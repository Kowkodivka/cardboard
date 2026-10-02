import { createSignal } from "solid-js";
import IconPaperclip from "~icons/tabler/paperclip";

export default function ReplyForm(props) {
  const [comment, setComment] = createSignal("");
  const [pending, setPending] = createSignal(false);

  const submit = async (e) => {
    e.preventDefault();
    const content = comment().trim();
    if (!content || pending()) return;

    setPending(true);
    try {
      await props.onSubmit(content);
      setComment("");
    } finally {
      setPending(false);
    }
  };

  return (
    <form onSubmit={submit} class="border border-neutral p-3 mt-2">
      <textarea
        value={comment()}
        onInput={(e) => setComment(e.currentTarget.value)}
        placeholder="comment"
        rows={2}
        class="textarea w-full text-xs resize-none focus:outline-none border-neutral bg-base-100"
      />
      <div class="flex items-center justify-between mt-1.5">
        <label class="flex items-center gap-1.5 text-[11px] text-base-content/50 cursor-pointer">
          <IconPaperclip size={14} />
          attach
          <input type="file" class="hidden" />
        </label>
        <button type="submit" class="btn btn-primary btn-xs" disabled={pending()}>
          {pending() ? "..." : "reply"}
        </button>
      </div>
    </form>
  );
}
