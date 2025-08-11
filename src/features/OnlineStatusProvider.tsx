import { PropsWithChildren, createContext, useContext, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";

type OnlineStatusContextType = {
  isOnline: boolean;
  setIsOnline: (isOnline: boolean) => void;
};

const OnlineStatusContext = createContext<OnlineStatusContextType | undefined>(
  undefined
);

export const OnlineStatusProvider = ({ children }: PropsWithChildren) => {
  const [isOnline, setIsOnline] = useState(false);

  // toggle online/offline mode with "cmd + o"
  useHotkeys(
    "ctrl+o",
    () => {
      setIsOnline((prev) => !prev);
    },
    {
      preventDefault: true,
      enableOnFormTags: ["INPUT", "TEXTAREA"],
    }
  );

  return (
    <OnlineStatusContext.Provider
      value={{
        isOnline,
        setIsOnline,
      }}
    >
      {children}
    </OnlineStatusContext.Provider>
  );
};

export const useOnlineStatus = () => {
  const context = useContext(OnlineStatusContext);
  if (context === undefined) {
    throw new Error(
      "useOnlineStatus must be used within a OnlineStatusProvider"
    );
  }
  return context;
};
