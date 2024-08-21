import { ComponentChildren } from "preact";

interface Props {
  text: string;
  checked?: boolean;
  disabled?: boolean;
  onChange?: (e: Event) => void;
  children?: ComponentChildren;
}

export default function CheckBox(props: Props) {
  return (
    <label className="cursor-pointer label justify-normal">
      <input
        type="checkbox"
        defaultChecked={props.checked}
        disabled={props.disabled}
        onChange={(e) => props.onChange?.(e)}
        className="checkbox mr-4"
      />
      <span>{props.text}</span>
      {props.children}
    </label>
  );
}