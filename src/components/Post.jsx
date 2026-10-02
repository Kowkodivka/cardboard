import { Show, For } from "solid-js";

export default function Post(props) {
  const lines = () => props.body.split("\n");

  return (
    <article
      id={`post-${props.id}`}
      class="border border-neutral p-3 text-xs leading-relaxed"
      classList={{
        "bg-base-200 mb-1.5": props.isOp,
        "bg-base-100 ml-6 mb-1.5": !props.isOp,
      }}
    >
      <header class="flex items-baseline gap-2 flex-wrap mb-1.5">
        <Show when={props.subject}>
          <span class="font-bold">{props.subject}</span>
        </Show>
        <span class="text-secondary">{props.name ?? "Anonymous"}</span>
        <span class="text-base-content/50">No.{props.id}</span>
        <Show when={props.replyTo}>
          <a href={`#post-${props.replyTo}`} class="text-accent">
            &gt;&gt;{props.replyTo}
          </a>
        </Show>
        <span class="text-base-content/50 ml-auto">{props.date}</span>
      </header>
      <p>
        <For each={lines()}>
          {(line) => (
            <>
              <span classList={{ "text-primary": line.trimStart().startsWith(">") }}>{line}</span>
              <br />
            </>
          )}
        </For>
      </p>
    </article>
  );
}
