import { createSignal } from "solid-js";

export default function NewThreadForm(props) {
  const [content, setContent] = createSignal("");
  const [pending, setPending] = createSignal(false);

  const submit = async (e) => {
    e.preventDefault();
    const value = content().trim();
    if (!value || pending()) return;

    setPending(true);
    try {
      await props.onSubmit(value);
      setContent("");
    } finally {
      setPending(false);
    }
  };

  return (
    <form onSubmit={submit} class="border border-neutral bg-base-200 p-3 mb-4">
      <textarea
        value={content()}
        onInput={(e) => setContent(e.currentTarget.value)}
        placeholder="о чем тред?"
        rows={3}
        class="textarea w-full text-xs resize-none focus:outline-none border-neutral bg-base-100"
      />
      <div class="flex items-center justify-end gap-2 mt-1.5">
        <button type="button" class="btn btn-ghost btn-xs" onClick={() => props.onCancel?.()}>
          отмена
        </button>
        <button type="submit" class="btn btn-primary btn-xs" disabled={pending()}>
          {pending() ? "..." : "создать"}
        </button>
      </div>
    </form>
  );
}
