import { ComponentChildren } from "preact";

interface Props {
  isOpen: boolean;
  title?: string;
  message: string;
  children?: ComponentChildren;
  isDismissable?: boolean;
  onClose?: () => void;
}

export default function Modal(props: Props) {
  return (
    <dialog className={`modal overflow-hidden backdrop-blur-sm backdrop-brightness-90 ${props.isOpen ? 'modal-open' : ''}`}>
      <div className="modal-box text-left">
        <h3 className="font-bold text-lg">{props.title}</h3>
        <p className="pt-4">{props.message}</p>
        {props.children}
        {
          props.isDismissable &&
          <div className="modal-action">
            <form method="dialog">
              <button className="btn" onClick={props.onClose}>Cerrar</button>
            </form>
          </div>
        }
      </div>
    </dialog>
  );
}