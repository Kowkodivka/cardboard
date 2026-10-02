import IconMessage from "~icons/tabler/message";

function formatTime(isoString) {
  if (!isoString) return "";
  const date = new Date(isoString);
  const diffMinutes = Math.floor((new Date() - date) / 60000);

  if (diffMinutes < 1) return "только что";
  if (diffMinutes < 60) return `${diffMinutes}м`;
  const diffHours = Math.floor(diffMinutes / 60);
  if (diffHours < 24) return `${diffHours}ч`;
  return `${Math.floor(diffHours / 24)}д`;
}

export default function ThreadCard(props) {
  const shortId = () => props.id.slice(0, 8);

  return (
    <a
      href={`/thread/${props.id}`}
      class="relative flex flex-col justify-between border border-neutral bg-base-200 p-3 hover:border-primary transition-colors min-h-27.5"
    >
      <div class="min-w-0">
        <div class="flex items-center justify-between text-[11px] text-base-content/50 mb-1.5 font-mono">
          <span class="text-accent truncate max-w-[70%]">{props.author_tripcode}</span>
          <span>#{shortId()}</span>
        </div>

        <p class="text-xs text-base-content/90 line-clamp-3 leading-snug wrap-break-word whitespace-pre-line">
          {props.content}
        </p>
      </div>

      <div class="flex items-center gap-2.5 text-[11px] text-base-content/50 border-t border-neutral mt-3 pt-1.5">
        <span class="flex items-center gap-1 font-mono">
          <IconMessage size={12} />
          {props.reply_count}
        </span>

        <span class="ml-auto text-[10px]">
          {formatTime(props.last_bumped_at || props.created_at)}
        </span>
      </div>
    </a>
  );
}
