import { Title } from "@solidjs/meta";
import { For, Show, Loading, createMemo, createSignal, action, refresh } from "solid-js";
import Header from "@/components/Header";
import ThreadCard from "@/components/ThreadCard";
import NewThreadForm from "@/components/NewThreadForm";
import { api } from "@/lib/api";
import IconPlus from "~icons/tabler/plus";

const DEFAULT_BOARD = { slug: "b", name: "general" };

export default function Home() {
  const boards = createMemo(() => api.listBoards());

  const createDefaultBoard = action(function* () {
    yield api.createBoard(DEFAULT_BOARD);
    refresh(boards);
  });

  return (
    <>
      <Title>Cardboard</Title>
      <main class="min-h-dvh bg-base-100 flex flex-col items-center px-4 sm:px-6 py-6 sm:py-8">
        <div class="w-full max-w-5xl flex flex-col flex-1">
          <Header />

          <Loading
            fallback={
              <div class="flex items-center justify-center py-16">
                <span class="loading loading-spinner loading-md"></span>
              </div>
            }
          >
            <Show
              when={boards().length > 0}
              fallback={
                <div class="flex flex-col items-center gap-2 py-16 text-center">
                  <p class="text-sm text-base-content/60">досок пока нет</p>
                  <button class="btn btn-primary btn-sm" onClick={() => createDefaultBoard()}>
                    создать доску "{DEFAULT_BOARD.name}"
                  </button>
                </div>
              }
            >
              <BoardView board={boards()[0]} />
            </Show>
          </Loading>
        </div>
      </main>
    </>
  );
}

function BoardView(props) {
  const threads = createMemo(() => api.listThreads(props.board.id));
  const [showForm, setShowForm] = createSignal(false);

  const createThread = action(function* (content) {
    yield api.createThread(props.board.id, { content });
    setShowForm(false);
    refresh(threads);
  });

  return (
    <>
      <div class="flex items-center gap-2 my-6">
        <span class="grow min-w-0 text-sm font-bold text-primary">/{props.board.slug}/</span>

        <button
          class="btn btn-primary btn-sm shrink-0 px-2.5 sm:px-3 gap-1"
          onClick={() => setShowForm((v) => !v)}
        >
          <IconPlus class="w-4 h-4" />
          <span class="hidden sm:inline">новый тред</span>
        </button>
      </div>

      <Show when={showForm()}>
        <NewThreadForm
          onSubmit={(content) => createThread(content)}
          onCancel={() => setShowForm(false)}
        />
      </Show>

      <Loading
        fallback={
          <div class="flex items-center justify-center py-16">
            <span class="loading loading-spinner loading-md"></span>
          </div>
        }
      >
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <For
            each={threads()}
            fallback={<p class="text-sm text-base-content/50 py-8">тредов пока нет</p>}
          >
            {(t) => <ThreadCard {...t} />}
          </For>
        </div>

        <footer class="mt-auto pt-6 pb-2 border-t border-base-content/10">
          <div class="flex items-center justify-between text-[11px] text-base-content/50">
            <span>{threads().length} тредов</span>
          </div>
        </footer>
      </Loading>
    </>
  );
}
