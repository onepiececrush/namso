import { create } from 'zustand';
import { AppStore, CardData } from './types';

export const useStore = create<AppStore>((set) => ({
  cards: [],
  result: '',
  loading: false,
  error: null,
  
  setCards: (cards: CardData[]) => set({ cards }),
  setResult: (result: string) => set({ result }),
  setLoading: (loading: boolean) => set({ loading }),
  setError: (error: string | null) => set({ error }),
  clearAll: () => set({ cards: [], result: '', error: null }),
}));


