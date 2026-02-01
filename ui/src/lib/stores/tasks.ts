import { writable } from 'svelte/store';
import type { Task, CreateTaskRequest, UpdateTaskRequest, AgentRun, AgentRole } from '$lib/api/types';
import { tasksApi } from '$lib/api/client';

function createTasksStore() {
  const { subscribe, set, update } = writable<Task[]>([]);

  return {
    subscribe,
    load: async (projectId: string) => {
      const tasks = await tasksApi.list(projectId);
      set(tasks);
      return tasks;
    },
    add: async (projectId: string, data: CreateTaskRequest) => {
      const task = await tasksApi.create(projectId, data);
      update(tasks => [task, ...tasks]);
      return task;
    },
    update: async (id: string, data: UpdateTaskRequest) => {
      const task = await tasksApi.update(id, data);
      update(tasks => tasks.map(t => t.id === id ? task : t));
      return task;
    },
    remove: async (id: string) => {
      await tasksApi.delete(id);
      update(tasks => tasks.filter(t => t.id !== id));
    },
    assignAgent: async (id: string, role: AgentRole) => {
      const run = await tasksApi.assignAgent(id, { agent_role: role });
      return run;
    },
    getRuns: async (id: string): Promise<AgentRun[]> => {
      return tasksApi.getRuns(id);
    },
    clear: () => set([])
  };
}

export const tasks = createTasksStore();
