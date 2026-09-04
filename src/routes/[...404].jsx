import { Title } from "@solidjs/meta";

import { httpStatus } from "@solidjs/web";

export const route = {
  preload: () => httpStatus(404),
};

export default function NotFound() {
  return (
    <>
      <Title>Not Found</Title>
      <main class="min-h-screen bg-base-100 flex items-center justify-center px-4">
        <div class="w-full max-w-md border border-neutral">
          <div class="px-5 py-8 text-center">
            <p class="text-3xl text-accent mb-2">404</p>
            <p class="text-base-content mb-1">страница не найдена</p>
            <p class="text-xs text-base-content/50">возможно, её удалили или адрес неверный</p>
          </div>

          <div class="px-5 py-4 border-t border-neutral flex items-center justify-end">
            <a href="/" class="btn btn-outline btn-primary btn-sm">
              на главную
            </a>
          </div>
        </div>
      </main>
    </>
  );
}
