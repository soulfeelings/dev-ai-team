-- Add Railway deployment fields to projects
ALTER TABLE projects ADD COLUMN railway_project_id TEXT;
ALTER TABLE projects ADD COLUMN railway_service_id TEXT;
ALTER TABLE projects ADD COLUMN railway_environment_id TEXT;
ALTER TABLE projects ADD COLUMN deployment_url TEXT;
ALTER TABLE projects ADD COLUMN deployment_status TEXT DEFAULT 'not_deployed';
