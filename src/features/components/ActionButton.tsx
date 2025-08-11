import { PropsWithChildren } from "react";
import LoaderIcon from "./LoaderIcon";

interface Props {
  isLoading: boolean;
  onClick: () => void;
  ariaLabel?: string;
  className?: string;
}

function ActionButton({
  isLoading,
  onClick,
  children,
  ariaLabel,
  className,
}: PropsWithChildren<Props>) {
  const handleClick = () => {
    if (!isLoading) {
      onClick();
    }
  };

  return (
    <button
      className={className}
      onClick={handleClick}
      aria-label={ariaLabel}
      aria-disabled={isLoading}
      aria-busy={isLoading}
    >
      {isLoading ? (
        <LoaderIcon className="row-span-2 text-blue-500 animate-spin" />
      ) : (
        children
      )}
    </button>
  );
}

export default ActionButton;
