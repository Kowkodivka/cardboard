const API_BASE = import.meta.env.VITE_API_URL ?? "/api";

async function request(path, options = {}) {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...options,
  });

  if (!res.ok) {
    const body = await res.json().catch(() => null);
    throw new Error(body?.error ?? `request failed: ${res.status}`);
  }

  if (res.status === 204) return null;
  return res.json();
}

export const api = {
  listBoards: () => request("/boards/list"),
  createBoard: (payload) =>
    request("/boards/create", { method: "POST", body: JSON.stringify(payload) }),
  getBoard: (slug) => request(`/boards/${slug}/get`),

  listThreads: (boardId) => request(`/boards/${boardId}/posts/list`),
  createThread: (boardId, payload) =>
    request(`/boards/${boardId}/posts/create`, {
      method: "POST",
      body: JSON.stringify(payload),
    }),

  listReplies: (postId) => request(`/posts/${postId}/replies/list`),
  createReply: (postId, payload) =>
    request(`/posts/${postId}/replies/create`, {
      method: "POST",
      body: JSON.stringify(payload),
    }),
};
