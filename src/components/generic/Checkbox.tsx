import type { JSX } from 'solid-js';

interface Props {
  text?: string;
  checked?: boolean;
  disabled?: boolean;
  onChange?: () => void;
  children?: JSX.Element;
}

export default function Checkbox({ text, checked = false, disabled = false, onChange, children }: Props) {
  return (
    <label class="cursor-pointer label justify-normal">
      <input
        type="checkbox"
        checked={checked}
        disabled={disabled}
        onChange={onChange}
        class="checkbox mr-4"
      />
      <span>{text}</span>
      {children}
    </label>
  );
}
