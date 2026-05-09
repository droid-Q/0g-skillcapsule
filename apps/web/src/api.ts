import type {
  Capsule,
  CapsuleRun,
  CreateCapsulePayload,
  CreateRunPayload,
  PublishCapsulePayload
} from "./types";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "http://127.0.0.1:18089";

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, init);
  if (!response.ok) {
    const body = await response.json().catch(() => ({ error: response.statusText }));
    throw new Error(body.error ?? "Request failed");
  }
  return response.json() as Promise<T>;
}

export function listCapsules(): Promise<Capsule[]> {
  return request<Capsule[]>("/api/capsules");
}

export function getCapsule(id: string): Promise<Capsule> {
  return request<Capsule>(`/api/capsules/${id}`);
}

export function createCapsule(payload: CreateCapsulePayload): Promise<Capsule> {
  return request<Capsule>("/api/capsules", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(payload)
  });
}

export function publishCapsule(id: string, payload: PublishCapsulePayload): Promise<Capsule> {
  return request<Capsule>(`/api/capsules/${id}/publish`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(payload)
  });
}

export function runCapsule(id: string, payload: CreateRunPayload): Promise<CapsuleRun> {
  return request<CapsuleRun>(`/api/capsules/${id}/run`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(payload)
  });
}

export function getRun(id: string): Promise<CapsuleRun> {
  return request<CapsuleRun>(`/api/runs/${id}`);
}

