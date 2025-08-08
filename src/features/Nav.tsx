import { NavLink } from "react-router";

function Nav() {
  return (
    <nav className="flex justify-center gap-4 text-sm text-zinc-400 mb-4">
      <NavLink
        to="/home"
        className={({ isActive }) => (isActive ? "text-amber-300" : "")}
      >
        search
      </NavLink>

      <NavLink
        to="/player"
        className={({ isActive }) => (isActive ? "text-amber-300" : "")}
      >
        player
      </NavLink>
    </nav>
  );
}

export default Nav;
