import { useState } from 'react';
import { Copy, Save, Trash2, Info, Loader2, CheckCircle } from 'lucide-react';
import { useStore } from '../lib/store';
import { copyToClipboard, saveToFile } from '../lib/utils';
import type { ExportFormat } from '../lib/types';

export function ResultPanel() {
  const { result, loading, error, clearAll, cards } = useStore();
  const [copied, setCopied] = useState(false);
  const [saveFormat, setSaveFormat] = useState<ExportFormat>('JSON');
  
  const handleCopy = async () => {
    if (!result) return;
    
    try {
      await copyToClipboard(result);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (error) {
      console.error('Copy failed:', error);
    }
  };
  
  const handleSave = async () => {
    if (!result) return;
    
    try {
      await saveToFile(result, saveFormat);
    } catch (error) {
      console.error('Save failed:', error);
    }
  };
  
  const handleClear = () => {
    if (confirm('确定要清空结果吗？')) {
      clearAll();
    }
  };
  
  const handleAbout = () => {
    alert(`💳 NamsoGen Desktop v2.0

🎯 专业的信用卡测试号码生成器

📋 功能特点:
• 🌐 支持多种信用卡网络
• 📄 多种导出格式
• 🔍 信用卡号码验证工具
• 👤 虚假用户数据生成
• 📝 Lorem Ipsum文本生成
• 🚀 快速批量生成

⚠️ 重要声明:
生成的卡号仅用于开发测试目的，不可用于实际交易。
请勿将此工具用于任何非法用途。

© 2025 NamsoGen - 仅供合法测试使用`);
  };
  
  return (
    <div className="h-full flex flex-col bg-white rounded-lg border border-gray-200 overflow-hidden">
      {/* 标题栏 */}
      <div className="bg-gradient-to-r from-blue-600 to-purple-600 text-white p-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold flex items-center gap-2">
            📊 生成结果
          </h2>
          {cards.length > 0 && (
            <span className="bg-white text-blue-600 px-3 py-1 rounded-full text-sm font-semibold">
              {cards.length} 张卡片
            </span>
          )}
        </div>
      </div>
      
      {/* 结果显示区域 */}
      <div className="flex-1 overflow-hidden relative">
        {loading ? (
          <div className="h-full flex items-center justify-center">
            <div className="text-center">
              <Loader2 className="w-12 h-12 text-blue-600 animate-spin mx-auto mb-4" />
              <p className="text-gray-600 font-medium">正在生成...</p>
            </div>
          </div>
        ) : error ? (
          <div className="h-full flex items-center justify-center">
            <div className="text-center p-6">
              <div className="text-6xl mb-4">❌</div>
              <p className="text-red-600 font-medium text-lg">{error}</p>
            </div>
          </div>
        ) : result ? (
          <textarea
            value={result}
            readOnly
            className="w-full h-full p-6 font-mono text-sm bg-gray-50 border-0 resize-none focus:outline-none"
            spellCheck={false}
          />
        ) : (
          <div className="h-full flex items-center justify-center">
            <div className="text-center p-6">
              <div className="text-6xl mb-4">💳</div>
              <p className="text-gray-500 text-lg">等待生成数据...</p>
              <p className="text-gray-400 text-sm mt-2">
                在左侧选择模式并点击生成按钮
              </p>
            </div>
          </div>
        )}
      </div>
      
      {/* 操作按钮栏 */}
      <div className="border-t border-gray-200 p-4 bg-gray-50">
        <div className="grid grid-cols-2 gap-3 mb-3">
          <button
            onClick={handleCopy}
            disabled={!result || loading}
            className="flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200"
          >
            {copied ? (
              <>
                <CheckCircle className="w-4 h-4" />
                已复制
              </>
            ) : (
              <>
                <Copy className="w-4 h-4" />
                📋 复制结果
              </>
            )}
          </button>
          
          <button
            onClick={handleClear}
            disabled={!result || loading}
            className="flex items-center justify-center gap-2 bg-gray-600 hover:bg-gray-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200"
          >
            <Trash2 className="w-4 h-4" />
            🗑️ 清空
          </button>
        </div>
        
        <div className="grid grid-cols-2 gap-3">
          <div className="flex gap-2">
            <select
              value={saveFormat}
              onChange={(e) => setSaveFormat(e.target.value as ExportFormat)}
              className="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option value="CARD">TXT</option>
              <option value="CSV">CSV</option>
              <option value="JSON">JSON</option>
              <option value="XML">XML</option>
              <option value="SQL">SQL</option>
            </select>
            <button
              onClick={handleSave}
              disabled={!result || loading}
              className="flex items-center justify-center gap-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200"
            >
              <Save className="w-4 h-4" />
              💾
            </button>
          </div>
          
          <button
            onClick={handleAbout}
            className="flex items-center justify-center gap-2 bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200"
          >
            <Info className="w-4 h-4" />
            ℹ️ 关于
          </button>
        </div>
      </div>
    </div>
  );
}


