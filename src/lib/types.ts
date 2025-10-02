// TypeScript 类型定义

export interface CardData {
  number: string;
  network: string;
  exp_month: string;
  exp_year: string;
  expiry: string;
  cvv?: string;
  balance?: number;
  currency?: string;
  bin: string;
}

export interface ValidationResult {
  valid: boolean;
  luhn_valid: boolean;
  network: string | null;
  length: number;
  reason: string;
}

export interface FakeUser {
  name: string;
  email: string;
  phone: string;
  address: string;
}

export type ExportFormat = 'CARD' | 'PIPE' | 'CSV' | 'JSON' | 'XML' | 'SQL';
export type CardNetwork = 'random' | 'visa' | 'mastercard' | 'amex' | 'discover' | 'unionpay' | 'diners';

export interface GenerateCardsParams {
  network: CardNetwork;
  quantity: number;
  exp_month: number | null;
  exp_year: number | null;
  include_cvv: boolean;
  include_balance: boolean;
  currency: string | null;
  bin_code: string | null;
}

export interface AppStore {
  cards: CardData[];
  result: string;
  loading: boolean;
  error: string | null;
  
  setCards: (cards: CardData[]) => void;
  setResult: (result: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearAll: () => void;
}


