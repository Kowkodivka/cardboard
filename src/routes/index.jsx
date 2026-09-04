import { Title } from "@solidjs/meta";
import { For } from "solid-js";

const boards = [
  { tag: "/b/", name: "разное", desc: "обо всём и ни о чём", count: 1204 },
  { tag: "/pr/", name: "программирование", desc: "код, языки, инструменты", count: 318 },
  { tag: "/sci/", name: "наука", desc: "физика, математика, данные", count: 96 },
  { tag: "/vg/", name: "игры", desc: "разработка и моддинг", count: 241 },
];

export default function Home() {
  return (
    <>
      <Title>Cardboard</Title>
      <main class="min-h-screen bg-base-100">
        <div class="max-w-2xl mx-auto px-4 py-10">
          <header class="flex items-baseline justify-between border-b border-neutral pb-4 mb-6">
            <span class="text-accent text-lg">
              cardboard<span class="text-base-content/40">://</span>
            </span>
            <nav class="flex gap-4 text-xs text-base-content/60">
              <a href="/" class="text-accent">
                все доски
              </a>
              <a href="/rules" class="hover:text-accent">
                правила
              </a>
              <a href="/faq" class="hover:text-accent">
                faq
              </a>
            </nav>
          </header>

          <p class="text-xs text-base-content/40 mb-2 ml-0.5">доски</p>

          <div class="border border-neutral">
            <For each={boards}>
              {(board) => (
                <a
                  href={`/${board.tag.replace(/\//g, "")}`}
                  class="grid grid-cols-[64px_1fr_auto] gap-3 items-baseline px-4 py-2.5 border-b border-neutral last:border-b-0 hover:bg-base-200 transition-colors"
                >
                  <span class="text-accent">{board.tag}</span>
                  <span class="text-base-content">
                    {board.name} <span class="text-xs text-base-content/40">— {board.desc}</span>
                  </span>
                  <span class="text-xs text-base-content/40 whitespace-nowrap">
                    {board.count} тредов
                  </span>
                </a>
              )}
            </For>
          </div>

          <footer class="mt-10 pt-3 border-t border-neutral flex justify-between text-xs text-base-content/40">
            <span>от kowkodivka</span>
            <span>сделано с любовью</span>
          </footer>
        </div>
      </main>
    </>
  );
}
