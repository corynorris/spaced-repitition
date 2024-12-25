import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { QueryClient } from "@tanstack/react-query";
import {
  BarChart2,
  BookOpen,
  LayoutGrid,
  Loader2,
  Settings,
} from "lucide-react";
import React from "react";
import "./App.css";
import { useAuth } from "./hooks/useAuth";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      cacheTime: 1000 * 60 * 30, // 30 minutes
      retry: 2,
    },
  },
});

const AppShell = () => {
  const { user, loading } = useAuth();
  const [isSyncing, setIsSyncing] = React.useState(false);

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Top Navigation */}
      <nav className="bg-white shadow-sm">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="flex h-16 justify-between items-center">
            <div className="flex items-center">
              <BookOpen className="h-8 w-8 text-blue-600" />
              <span className="ml-2 text-xl font-semibold">MemoryDeck</span>
            </div>

            {isSyncing && (
              <div className="flex items-center text-sm text-gray-500">
                <Loader2 className="h-4 w-4 animate-spin mr-2" />
                Syncing...
              </div>
            )}
          </div>
        </div>
      </nav>

      {/* Main Content Area */}
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* Quick Stats */}
          <Card>
            <CardHeader>
              <CardTitle>Due Today</CardTitle>
              <CardDescription>Cards waiting for review</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-blue-600">23</div>
            </CardContent>
          </Card>

          {/* Study Streak */}
          <Card>
            <CardHeader>
              <CardTitle>Study Streak</CardTitle>
              <CardDescription>Keep it going!</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-green-600">7 days</div>
            </CardContent>
          </Card>

          {/* Total Cards */}
          <Card>
            <CardHeader>
              <CardTitle>Total Cards</CardTitle>
              <CardDescription>Across all decks</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-purple-600">156</div>
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Bottom Navigation */}
      <nav className="fixed bottom-0 w-full bg-white border-t border-gray-200">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="flex justify-around py-3">
            <button className="flex flex-col items-center text-gray-600">
              <LayoutGrid className="h-6 w-6" />
              <span className="text-xs mt-1">Decks</span>
            </button>
            <button className="flex flex-col items-center text-gray-600">
              <BookOpen className="h-6 w-6" />
              <span className="text-xs mt-1">Study</span>
            </button>
            <button className="flex flex-col items-center text-gray-600">
              <BarChart2 className="h-6 w-6" />
              <span className="text-xs mt-1">Stats</span>
            </button>
            <button className="flex flex-col items-center text-gray-600">
              <Settings className="h-6 w-6" />
              <span className="text-xs mt-1">Settings</span>
            </button>
          </div>
        </div>
      </nav>
    </div>
  );
};

export default AppShell;
