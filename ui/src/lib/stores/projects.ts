import { writable } from 'svelte/store';
import type { Project } from '$lib/api/types';
import { projectsApi } from '$lib/api/client';

function createProjectsStore() {
  const { subscribe, set, update } = writable<Project[]>([]);

  return {
    subscribe,
    load: async () => {
      const projects = await projectsApi.list();
      set(projects);
      return projects;
    },
    add: async (data: { name: string; github_url: string; local_path?: string; default_branch?: string }) => {
      const project = await projectsApi.create(data);
      update(projects => [project, ...projects]);
      return project;
    },
    remove: async (id: string) => {
      await projectsApi.delete(id);
      update(projects => projects.filter(p => p.id !== id));
    },
    refresh: async () => {
      const projects = await projectsApi.list();
      set(projects);
    }
  };
}

export const projects = createProjectsStore();
