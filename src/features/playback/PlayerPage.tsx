import Logo from "../components/Logo";
import Nav from "../Nav";

function PlayerPage() {
  return (
    <main className="container mx-auto p-4 flex flex-col h-screen">
      <header className="relative">
        <Logo />
        <Nav />
      </header>

      <h1>hello player page</h1>
    </main>
  );
}

export default PlayerPage;
