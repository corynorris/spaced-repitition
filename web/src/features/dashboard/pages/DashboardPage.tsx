import { useAuth } from "../../auth/hooks/useAuth";

export function DashboardPage() {
  const { user, logout } = useAuth();

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <div className="flex items-center gap-4">
          <span>Welcome, {user?.email}</span>
          <button
            onClick={logout}
            className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
          >
            Logout
          </button>
        </div>
      </div>
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">Protected Content</h2>
        <p>This page is only visible to authenticated users.</p>
        <pre className="mt-4 p-4 bg-gray-100 rounded">
          {JSON.stringify(user, null, 2)}
        </pre>
      </div>
    </div>
  );
}
