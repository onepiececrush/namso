import { useState, useEffect } from 'react';
import { Zap, Building2, DollarSign } from 'lucide-react';
import { generateCards, exportCards, getNetworks, getCurrencies } from '../lib/utils';
import { useStore } from '../lib/store';
import type { CardNetwork, ExportFormat } from '../lib/types';

const EXPORT_FORMATS: ExportFormat[] = ['PIPE', 'CSV', 'JSON', 'XML', 'SQL'];
const MONTHS = Array.from({ length: 12 }, (_, i) => (i + 1).toString().padStart(2, '0'));
const QUANTITIES = [10, 50, 100, 500, 1000, 2000, 5000, 10000];

export function AdvancedMode() {
  const [network, setNetwork] = useState<CardNetwork>('random');
  const [format, setFormat] = useState<ExportFormat>('PIPE');
  const [month, setMonth] = useState<string>('random');
  const [year, setYear] = useState<string>('random');
  const [includeCvv, setIncludeCvv] = useState(true);
  const [includeBalance, setIncludeBalance] = useState(false);
  const [currency, setCurrency] = useState('USD');
  const [quantity, setQuantity] = useState(100);
  const [binCode, setBinCode] = useState('');
  const [networks, setNetworks] = useState<[string, string][]>([
    ['random', 'Random'],
    ['visa', 'Visa'],
    ['mastercard', 'Mastercard'],
    ['amex', 'American Express'],
    ['discover', 'Discover'],
    ['unionpay', 'UnionPay'],
    ['diners', 'Diners Club'],
  ]);
  const [currencies, setCurrenciesList] = useState<[string, string][]>([
    ['USD', 'United States Dollar'],
    ['EUR', 'Euro'],
    ['CNY', 'Chinese Yuan'],
    ['JPY', 'Japanese Yen'],
    ['GBP', 'British Pound'],
  ]);
  
  const { setCards, setResult, setLoading, setError } = useStore();
  
  useEffect(() => {
    getNetworks().then(setNetworks).catch(console.error);
    getCurrencies().then(setCurrenciesList).catch(console.error);
  }, []);
  
  const currentYear = new Date().getFullYear();
  const years = Array.from({ length: 9 }, (_, i) => currentYear + i);
  
  const handleGenerate = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const params = {
        network,
        quantity,
        exp_month: month === 'random' ? null : parseInt(month),
        exp_year: year === 'random' ? null : parseInt(year),
        include_cvv: includeCvv,
        include_balance: includeBalance,
        currency: includeBalance ? currency : null,
        bin_code: binCode.trim() || null,
      };
      
      console.log('高级模式生成参数:', params);
      
      const cards = await generateCards(params);
      
      console.log('高级模式生成的卡号:', cards);
      setCards(cards);
      
      const result = await exportCards(cards, format);
      console.log('高级模式导出结果:', result);
      setResult(result);
    } catch (error) {
      console.error('高级模式生成失败:', error);
      setError(error instanceof Error ? error.message : '生成失败');
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center gap-3 mb-6">
        <Zap className="w-6 h-6 text-purple-600" />
        <h2 className="text-2xl font-bold text-gray-800">高级模式</h2>
      </div>
      
      {/* BIN 码 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center gap-2">
          <Building2 className="w-4 h-4" />
          BIN 码 (可选)
        </label>
        <input
          type="text"
          value={binCode}
          onChange={(e) => setBinCode(e.target.value)}
          placeholder="例如: 559888039xxxxxxx"
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
        />
        <p className="mt-1 text-xs text-gray-500">
          💡 提示: 可使用 x 作为占位符，如: 559888039xxxxxxx
        </p>
      </div>
      
      {/* 网络选择 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          🌐 选择网络
        </label>
        <select
          value={network}
          onChange={(e) => setNetwork(e.target.value as CardNetwork)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
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
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
        >
          {EXPORT_FORMATS.map(f => (
            <option key={f} value={f}>{f}</option>
          ))}
        </select>
      </div>
      
      {/* 过期月份和年份 */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            📅 过期月份
          </label>
          <select
            value={month}
            onChange={(e) => setMonth(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
          >
            <option value="random">Random</option>
            {MONTHS.map(m => (
              <option key={m} value={m}>{m}</option>
            ))}
          </select>
        </div>
        
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            🗓️ 过期年份
          </label>
          <select
            value={year}
            onChange={(e) => setYear(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
          >
            <option value="random">Random</option>
            {years.map(y => (
              <option key={y} value={y}>{y}</option>
            ))}
          </select>
        </div>
      </div>
      
      {/* 选项 */}
      <div className="space-y-2">
        <div className="flex items-center gap-2">
          <input
            type="checkbox"
            id="adv-includeCvv"
            checked={includeCvv}
            onChange={(e) => setIncludeCvv(e.target.checked)}
            className="w-4 h-4 text-purple-600 rounded focus:ring-purple-500"
          />
          <label htmlFor="adv-includeCvv" className="text-sm font-medium text-gray-700">
            🔒 包含 CVV
          </label>
        </div>
        
        <div className="flex items-center gap-2">
          <input
            type="checkbox"
            id="adv-includeBalance"
            checked={includeBalance}
            onChange={(e) => setIncludeBalance(e.target.checked)}
            className="w-4 h-4 text-purple-600 rounded focus:ring-purple-500"
          />
          <label htmlFor="adv-includeBalance" className="text-sm font-medium text-gray-700">
            💰 包含余额
          </label>
        </div>
      </div>
      
      {/* 货币 */}
      {includeBalance && (
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center gap-2">
            <DollarSign className="w-4 h-4" />
            货币类型
          </label>
          <select
            value={currency}
            onChange={(e) => setCurrency(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
          >
            {currencies.map(([code, name]) => (
              <option key={code} value={code}>{code} - {name}</option>
            ))}
          </select>
        </div>
      )}
      
      {/* 数量 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          🔢 生成数量
        </label>
        <select
          value={quantity}
          onChange={(e) => setQuantity(parseInt(e.target.value))}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
        >
          {QUANTITIES.map(q => (
            <option key={q} value={q}>{q}</option>
          ))}
        </select>
      </div>
      
      {/* 生成按钮 */}
      <button
        onClick={handleGenerate}
        className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
      >
        <Zap className="w-5 h-5" />
        ⚡ 高级生成
      </button>
    </div>
  );
}


