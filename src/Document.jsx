import { HydrationScript } from "@solidjs/web";

export default function Document(props) {
  return (
    <html lang="ru">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
        <title>Cardboard</title>
        <HydrationScript />
      </head>
      <body class="text-center font-sans">{props.children}</body>
    </html>
  );
}
