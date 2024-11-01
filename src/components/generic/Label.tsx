import type { JSX } from 'solid-js';

interface Props {
  text: string;
  children?: JSX.Element;
}

export default function Label({ text, children }: Props) {
  return (
    <label class="form-control">
      <div class="label">
        <span class="label-text">{text}</span>
      </div>
      {children}
    </label>
  );
}
