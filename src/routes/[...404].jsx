import { Title } from "@solidjs/meta";
import { httpStatus } from "@solidjs/web";

export const route = {
  preload: () => httpStatus(404),
};

export default function NotFound() {
  return (
    <>
      <Title>Not Found</Title>
      <main class="min-h-dvh bg-base-100 flex items-center justify-center px-6 py-8">
        <div class="w-full max-w-md border border-neutral bg-base-200">
          <div class="px-6 py-8 text-left">
            <p class="text-5xl font-bold text-error mb-2 tracking-tight">404</p>
            <p class="text-sm font-medium mb-1.5">страница не найдена</p>
            <p class="text-[11px] leading-relaxed text-base-content/60">
              возможно ее удалили или адрес неверный
            </p>
          </div>

          <div class="flex items-center justify-start border-t border-neutral p-3">
            <a href="/" class="btn btn-primary btn-sm px-6">
              на главную
            </a>
          </div>
        </div>
      </main>
    </>
  );
}
