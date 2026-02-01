use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{AppError, AppResult};

const RAILWAY_API_URL: &str = "https://backboard.railway.app/graphql/v2";

#[derive(Debug, Clone)]
pub struct RailwayService {
    client: Client,
    api_token: String,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct ProjectCreateData {
    #[serde(rename = "projectCreate")]
    project_create: ProjectResult,
}

#[derive(Debug, Deserialize)]
struct ProjectResult {
    id: String,
    environments: Environments,
}

#[derive(Debug, Deserialize)]
struct Environments {
    edges: Vec<EnvironmentEdge>,
}

#[derive(Debug, Deserialize)]
struct EnvironmentEdge {
    node: EnvironmentNode,
}

#[derive(Debug, Deserialize)]
struct EnvironmentNode {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct ServiceCreateData {
    #[serde(rename = "serviceCreate")]
    service_create: ServiceResult,
}

#[derive(Debug, Deserialize)]
struct ServiceResult {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ServiceInstanceDeployData {
    #[serde(rename = "serviceInstanceDeploy")]
    service_instance_deploy: bool,
}

#[derive(Debug, Deserialize)]
struct DeploymentsData {
    deployments: DeploymentsResult,
}

#[derive(Debug, Deserialize)]
struct DeploymentsResult {
    edges: Vec<DeploymentEdge>,
}

#[derive(Debug, Deserialize)]
struct DeploymentEdge {
    node: DeploymentNode,
}

#[derive(Debug, Deserialize)]
pub struct DeploymentNode {
    pub id: String,
    pub status: String,
    #[serde(rename = "staticUrl")]
    pub static_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentInfo {
    pub railway_project_id: String,
    pub railway_service_id: String,
    pub railway_environment_id: String,
    pub deployment_url: Option<String>,
    pub status: String,
}

impl RailwayService {
    pub fn new(api_token: String) -> Self {
        Self {
            client: Client::new(),
            api_token,
        }
    }

    async fn execute<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> AppResult<T> {
        let response = self
            .client
            .post(RAILWAY_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Railway API request failed: {}", e)))?;

        let result: GraphQLResponse<T> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse Railway response: {}", e)))?;

        if let Some(errors) = result.errors {
            let msg = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(AppError::Internal(format!("Railway API error: {}", msg)));
        }

        result
            .data
            .ok_or_else(|| AppError::Internal("No data in Railway response".to_string()))
    }

    /// Create a new Railway project and service from a GitHub repo
    pub async fn deploy_from_github(
        &self,
        project_name: &str,
        github_repo: &str,
        branch: &str,
    ) -> AppResult<DeploymentInfo> {
        // Step 1: Create a new Railway project
        let create_project_query = r#"
            mutation projectCreate($input: ProjectCreateInput!) {
                projectCreate(input: $input) {
                    id
                    environments {
                        edges {
                            node {
                                id
                                name
                            }
                        }
                    }
                }
            }
        "#;

        let project_data: ProjectCreateData = self
            .execute(
                create_project_query,
                json!({
                    "input": {
                        "name": project_name
                    }
                }),
            )
            .await?;

        let railway_project_id = project_data.project_create.id;
        let environment = project_data
            .project_create
            .environments
            .edges
            .into_iter()
            .find(|e| e.node.name == "production")
            .or_else(|| None)
            .ok_or_else(|| AppError::Internal("No production environment found".to_string()))?;
        let railway_environment_id = environment.node.id;

        // Step 2: Create a service connected to GitHub repo
        let create_service_query = r#"
            mutation serviceCreate($input: ServiceCreateInput!) {
                serviceCreate(input: $input) {
                    id
                }
            }
        "#;

        let service_data: ServiceCreateData = self
            .execute(
                create_service_query,
                json!({
                    "input": {
                        "projectId": railway_project_id,
                        "name": project_name,
                        "source": {
                            "repo": github_repo
                        },
                        "branch": branch
                    }
                }),
            )
            .await?;

        let railway_service_id = service_data.service_create.id;

        Ok(DeploymentInfo {
            railway_project_id,
            railway_service_id,
            railway_environment_id,
            deployment_url: None,
            status: "deploying".to_string(),
        })
    }

    /// Trigger a new deployment for an existing service
    pub async fn redeploy(&self, service_id: &str, environment_id: &str) -> AppResult<()> {
        let query = r#"
            mutation serviceInstanceDeploy($serviceId: String!, $environmentId: String!) {
                serviceInstanceDeploy(serviceId: $serviceId, environmentId: $environmentId)
            }
        "#;

        let _: ServiceInstanceDeployData = self
            .execute(
                query,
                json!({
                    "serviceId": service_id,
                    "environmentId": environment_id
                }),
            )
            .await?;

        Ok(())
    }

    /// Get the latest deployment status and URL
    pub async fn get_deployment_status(
        &self,
        project_id: &str,
        service_id: &str,
        environment_id: &str,
    ) -> AppResult<DeploymentNode> {
        let query = r#"
            query deployments($projectId: String!, $serviceId: String!, $environmentId: String!) {
                deployments(
                    first: 1
                    input: {
                        projectId: $projectId
                        serviceId: $serviceId
                        environmentId: $environmentId
                    }
                ) {
                    edges {
                        node {
                            id
                            status
                            staticUrl
                        }
                    }
                }
            }
        "#;

        let data: DeploymentsData = self
            .execute(
                query,
                json!({
                    "projectId": project_id,
                    "serviceId": service_id,
                    "environmentId": environment_id
                }),
            )
            .await?;

        data.deployments
            .edges
            .into_iter()
            .next()
            .map(|e| e.node)
            .ok_or_else(|| AppError::NotFound("No deployments found".to_string()))
    }
}
