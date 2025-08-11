import { Outlet } from "react-router";
import Logo from "./components/Logo";
import OfflineIcon from "./components/OfflineIcon";
import OnlineIcon from "./components/OnlineIcon";
import Nav from "./Nav";
import { useOnlineStatus } from "./OnlineStatusProvider";
import { useDownloadStatus } from "./search/useDownloadStatus";

function MainLayout() {
  const { isQueueActive } = useDownloadStatus();
  const { isOnline, setIsOnline } = useOnlineStatus();

  const handleOnlineToggle = () => {
    setIsOnline(!isOnline);
  };

  return (
    <main className="container mx-auto p-4">
      <header className="relative">
        <Logo animated={isQueueActive} />
        <button
          onClick={handleOnlineToggle}
          className="absolute top-2 right-0 opacity-70 focus:opacity-100 hover:opacity-100"
        >
          {isOnline ? <OnlineIcon /> : <OfflineIcon />}
        </button>
        <Nav />
      </header>

      <Outlet />
    </main>
  );
}

export default MainLayout;
