import { invoke } from '@tauri-apps/api/core';
import type { CardData, ExportFormat, GenerateCardsParams, ValidationResult, FakeUser } from './types';

// Tauri 命令调用

export async function generateCards(params: GenerateCardsParams): Promise<CardData[]> {
  return await invoke('generate_cards', {
    network: params.network,
    quantity: params.quantity,
    exp_month: params.exp_month ?? null,
    exp_year: params.exp_year ?? null,
    include_cvv: params.include_cvv,
    include_balance: params.include_balance,
    currency: params.currency ?? null,
    bin_code: params.bin_code ?? null,
  });
}

export async function validateCard(card_number: string): Promise<ValidationResult> {
  return await invoke('validate_card', { card_number });
}

export async function exportCards(cards: CardData[], format: ExportFormat): Promise<string> {
  return await invoke('export_cards', { cards, format });
}

export async function generateUsers(count: number): Promise<FakeUser[]> {
  return await invoke('generate_users', { count });
}

export async function generateLorem(paragraphs: number): Promise<string> {
  return await invoke('generate_lorem', { paragraphs });
}

export async function getCurrencies(): Promise<[string, string][]> {
  return await invoke('get_currencies');
}

export async function getNetworks(): Promise<[string, string][]> {
  return await invoke('get_networks');
}

// 工具函数

export async function copyToClipboard(text: string): Promise<void> {
  try {
    // 使用浏览器 API 作为备用方案
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
    } else {
      // 创建临时文本区域
      const textArea = document.createElement('textarea');
      textArea.value = text;
      textArea.style.position = 'fixed';
      textArea.style.left = '-999999px';
      textArea.style.top = '-999999px';
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand('copy');
      textArea.remove();
    }
  } catch (error) {
    console.error('Failed to copy to clipboard:', error);
    throw new Error('复制到剪贴板失败');
  }
}

export async function saveToFile(content: string, format: ExportFormat): Promise<void> {
  try {
    const extensions: Record<ExportFormat, string> = {
      'CARD': 'txt',
      'PIPE': 'txt',
      'CSV': 'csv',
      'JSON': 'json',
      'XML': 'xml',
      'SQL': 'sql',
    };
    
    const ext = extensions[format];
    const fileName = `namso-gen-${Date.now()}.${ext}`;
    
    // 使用浏览器下载 API 作为备用方案
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = fileName;
    a.style.display = 'none';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('Failed to save file:', error);
    throw new Error('保存文件失败');
  }
}

export function formatCardDisplay(cards: CardData[]): string {
  return cards.map((card, i) => {
    let display = `🔖 卡片 #${i + 1}\n`;
    display += `💳 卡号: ${card.number}\n`;
    display += `🌐 网络: ${card.network}\n`;
    display += `📅 过期: ${card.expiry}\n`;
    
    if (card.cvv) {
      display += `🔒 CVV: ${card.cvv}\n`;
    }
    
    if (card.balance) {
      display += `💰 余额: ${card.balance} ${card.currency || 'USD'}\n`;
    }
    
    display += '─'.repeat(40);
    return display;
  }).join('\n');
}

export function getCurrentYear(): number {
  return new Date().getFullYear();
}

export function getYearOptions(count: number = 9): number[] {
  const currentYear = getCurrentYear();
  return Array.from({ length: count }, (_, i) => currentYear + i);
}


