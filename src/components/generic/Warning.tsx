interface Props {
  text: string;
  title?: string;
  buttonText?: string;
  onButtonClick?: () => void;
}

export default function Warning({ text, title = "Aviso", buttonText, onButtonClick }: Props) {
  return (
    <div role="alert" class="alert alert-warning not-prose">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-6 w-6 shrink-0 stroke-current"
        fill="none"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
        />
      </svg>
      <div>
        <h3 class="font-bold">{title || 'Aviso'}</h3>
        <div class="text-xs">{text}</div>
      </div>
      {buttonText && (
        <button class="btn btn-sm" onClick={() => onButtonClick?.()}>
          {buttonText}
        </button>
      )}
    </div>
  );
}