import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { For, Loading, createMemo, action, refresh } from "solid-js";
import Header from "@/components/Header";
import Post from "@/components/Post";
import ReplyForm from "@/components/Replyform";
import { api } from "@/lib/api";

function formatDate(isoString) {
  if (!isoString) return "";
  return new Date(isoString).toLocaleString("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export default function Thread() {
  const params = useParams();
  const posts = createMemo(() => api.listReplies(params.id));

  const reply = action(function* (content) {
    yield api.createReply(params.id, { content });
    refresh(posts);
  });

  return (
    <>
      <Title>Cardboard — thread</Title>
      <main class="min-h-dvh bg-base-100 flex flex-col items-center px-4 sm:px-6 py-6 sm:py-8">
        <div class="w-full max-w-3xl flex flex-col flex-1">
          <Header />

          <Loading
            fallback={
              <div class="flex items-center justify-center py-16">
                <span class="loading loading-spinner loading-md"></span>
              </div>
            }
          >
            <div class="flex flex-col mt-4">
              <For each={posts()}>
                {(p, i) => (
                  <Post
                    id={p.id.slice(0, 8)}
                    isOp={i() === 0}
                    name={p.author_tripcode}
                    date={formatDate(p.created_at)}
                    body={p.content}
                  />
                )}
              </For>
            </div>

            <ReplyForm onSubmit={(content) => reply(content)} />
          </Loading>
        </div>
      </main>
    </>
  );
}
