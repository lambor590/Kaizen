import { ComponentChildren } from "preact";

interface Props {
  text: string;
  children?: ComponentChildren;
}

export default function Label(props: Props) {
  return (
    <label className="form-control">
      <div className="label">
        <span className="label-text">{props.text}</span>
      </div>
      {props.children}
    </label>
  );
}