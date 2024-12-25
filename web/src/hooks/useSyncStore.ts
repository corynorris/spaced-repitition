interface SyncStore {
  pendingChanges: Change[];
  addChange: (change: Change) => void;
  syncChanges: () => Promise<void>;
}

const useSyncStore = create<SyncStore>((set, get) => ({
  pendingChanges: [],
  addChange: (change) => {
    set((state) => ({
      pendingChanges: [...state.pendingChanges, change],
    }));
  },
  syncChanges: async () => {
    const { pendingChanges } = get();
    await api.sync.pushChanges(pendingChanges);
    set({ pendingChanges: [] });
  },
}));
