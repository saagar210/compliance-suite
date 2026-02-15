import { Link, useLocation } from "react-router-dom";
import { useUiStore } from "../../state/uiStore";

const navItems = [
  { path: "/", label: "Import", icon: "📥" },
  { path: "/map", label: "Map Columns", icon: "🗺️" },
  { path: "/answer-bank", label: "Answer Bank", icon: "📚" },
  { path: "/review", label: "Review", icon: "✓" },
  { path: "/export", label: "Export", icon: "📤" },
];

export function Sidebar() {
  const location = useLocation();
  const sidebarOpen = useUiStore((state) => state.sidebarOpen);

  if (!sidebarOpen) {
    return null;
  }

  return (
    <aside className="w-64 bg-secondary border-r border-border">
      <div className="p-6">
        <h1 className="text-xl font-bold text-foreground mb-6">Questionnaire Autopilot</h1>
        <nav className="space-y-2">
          {navItems.map((item) => {
            const isActive = location.pathname === item.path;
            return (
              <Link
                key={item.path}
                to={item.path}
                className={`flex items-center gap-3 px-4 py-3 rounded-md transition-colors ${
                  isActive
                    ? "bg-primary text-primary-foreground"
                    : "text-foreground hover:bg-accent hover:text-accent-foreground"
                }`}
              >
                <span className="text-lg">{item.icon}</span>
                <span className="font-medium">{item.label}</span>
              </Link>
            );
          })}
        </nav>
      </div>
    </aside>
  );
}
