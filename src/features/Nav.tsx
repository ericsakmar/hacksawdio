import { useHotkeys } from "react-hotkeys-hook";
import { NavLink, useNavigate } from "react-router";

function Nav() {
  const navigate = useNavigate();

  useHotkeys("ctrl+s", () => {
    navigate("/search");
  });

  useHotkeys("ctrl+l", () => {
    navigate("/player");
  });

  return (
    <nav className="flex justify-center gap-4 text-sm text-zinc-400 mb-4">
      <NavLink
        to="/search"
        className={({ isActive }) => (isActive ? "text-amber-300" : "")}
      >
        search
      </NavLink>

      <NavLink
        to="/player"
        className={({ isActive }) => (isActive ? "text-amber-300" : "")}
      >
        listen
      </NavLink>
    </nav>
  );
}

export default Nav;
