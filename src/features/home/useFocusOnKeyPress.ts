import { useEffect } from "react";

export function useFocusOnKeyPress(
  key: string,
  ref: React.RefObject<HTMLElement>
) {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === key) {
        event.preventDefault();
        ref.current?.focus();

        if (ref.current instanceof HTMLInputElement) {
          ref.current.select();
        }
      }

      if (event.key === "Escape") {
        event.preventDefault();
        ref.current?.blur();
      }
    };

    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [key, ref]);
}
