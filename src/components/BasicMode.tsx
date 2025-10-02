import { useState, useEffect } from 'react';
import { CreditCard, Calendar, Hash, Lock } from 'lucide-react';
import { generateCards, exportCards, getNetworks } from '../lib/utils';
import { useStore } from '../lib/store';
import type { CardNetwork, ExportFormat } from '../lib/types';

const EXPORT_FORMATS: ExportFormat[] = ['CARD', 'PIPE', 'CSV', 'JSON', 'XML', 'SQL'];
const MONTHS = Array.from({ length: 12 }, (_, i) => (i + 1).toString().padStart(2, '0'));
const QUANTITIES = [10, 50, 100, 500];

export function BasicMode() {
  const [network, setNetwork] = useState<CardNetwork>('random');
  const [format, setFormat] = useState<ExportFormat>('CARD');
  const [month, setMonth] = useState<string>('random');
  const [year, setYear] = useState<string>('random');
  const [includeCvv, setIncludeCvv] = useState(true);
  const [quantity, setQuantity] = useState(10);
  const [networks, setNetworks] = useState<[string, string][]>([
    ['random', 'Random'],
    ['visa', 'Visa'],
    ['mastercard', 'Mastercard'],
    ['amex', 'American Express'],
    ['discover', 'Discover'],
    ['unionpay', 'UnionPay'],
    ['diners', 'Diners Club'],
  ]);
  
  const { setCards, setResult, setLoading, setError } = useStore();
  
  useEffect(() => {
    getNetworks().then(setNetworks).catch(console.error);
  }, []);
  
  const currentYear = new Date().getFullYear();
  const years = Array.from({ length: 9 }, (_, i) => currentYear + i);
  
  const handleGenerate = async () => {
    try {
      setLoading(true);
      setError(null);
      
      console.log('开始生成卡号，参数:', {
        network,
        quantity,
        exp_month: month === 'random' ? null : parseInt(month),
        exp_year: year === 'random' ? null : parseInt(year),
        include_cvv: includeCvv,
        include_balance: false,
        currency: null,
        bin_code: null,
      });
      
      const cards = await generateCards({
        network,
        quantity,
        exp_month: month === 'random' ? null : parseInt(month),
        exp_year: year === 'random' ? null : parseInt(year),
        include_cvv: includeCvv,
        include_balance: false,
        currency: null,
        bin_code: null,
      });
      
      console.log('生成的卡号:', cards);
      setCards(cards);
      
      const result = await exportCards(cards, format);
      console.log('导出结果:', result);
      setResult(result);
    } catch (error) {
      console.error('生成失败:', error);
      setError(error instanceof Error ? error.message : '生成失败');
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center gap-3 mb-6">
        <CreditCard className="w-6 h-6 text-blue-600" />
        <h2 className="text-2xl font-bold text-gray-800">基础模式</h2>
      </div>
      
      {/* 网络选择 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          🌐 选择网络
        </label>
        <select
          value={network}
          onChange={(e) => setNetwork(e.target.value as CardNetwork)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
          {networks.map(([value, label]) => (
            <option key={value} value={value}>{label}</option>
          ))}
        </select>
      </div>
      
      {/* 导出格式 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          📄 导出格式
        </label>
        <select
          value={format}
          onChange={(e) => setFormat(e.target.value as ExportFormat)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
          {EXPORT_FORMATS.map(f => (
            <option key={f} value={f}>{f}</option>
          ))}
        </select>
      </div>
      
      {/* 过期月份 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center gap-2">
          <Calendar className="w-4 h-4" />
          过期月份
        </label>
        <select
          value={month}
          onChange={(e) => setMonth(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
          <option value="random">Random</option>
          {MONTHS.map(m => (
            <option key={m} value={m}>{m}</option>
          ))}
        </select>
      </div>
      
      {/* 过期年份 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center gap-2">
          <Calendar className="w-4 h-4" />
          过期年份
        </label>
        <select
          value={year}
          onChange={(e) => setYear(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
          <option value="random">Random</option>
          {years.map(y => (
            <option key={y} value={y}>{y}</option>
          ))}
        </select>
      </div>
      
      {/* CVV 选项 */}
      <div className="flex items-center gap-2">
        <input
          type="checkbox"
          id="includeCvv"
          checked={includeCvv}
          onChange={(e) => setIncludeCvv(e.target.checked)}
          className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
        />
        <label htmlFor="includeCvv" className="text-sm font-medium text-gray-700 flex items-center gap-2">
          <Lock className="w-4 h-4" />
          包含 CVV
        </label>
      </div>
      
      {/* 数量 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center gap-2">
          <Hash className="w-4 h-4" />
          生成数量
        </label>
        <select
          value={quantity}
          onChange={(e) => setQuantity(parseInt(e.target.value))}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        >
          {QUANTITIES.map(q => (
            <option key={q} value={q}>{q}</option>
          ))}
        </select>
      </div>
      
      {/* 生成按钮 */}
      <button
        onClick={handleGenerate}
        className="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
      >
        <CreditCard className="w-5 h-5" />
        🚀 生成信用卡
      </button>
    </div>
  );
}


