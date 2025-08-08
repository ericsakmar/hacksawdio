import Logo from "../components/Logo";
import Nav from "../Nav";
import Player from "./Player";

function PlayerPage() {
  return (
    <main className="container mx-auto p-4 flex flex-col h-screen">
      <header className="relative">
        <Logo />
        <Nav />
      </header>

      <Player />
    </main>
  );
}

export default PlayerPage;
