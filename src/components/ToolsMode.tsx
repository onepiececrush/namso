import { useState } from 'react';
import { Search, Users, FileText } from 'lucide-react';
import { validateCard, generateUsers, generateLorem } from '../lib/utils';
import { useStore } from '../lib/store';

export function ToolsMode() {
  const [cardNumber, setCardNumber] = useState('');
  const [validationResult, setValidationResult] = useState<string>('');
  const [loremCount, setLoremCount] = useState(3);
  
  const { setResult, setLoading, setError } = useStore();
  
  const handleValidate = async () => {
    if (!cardNumber.trim()) {
      setValidationResult('⚠️ 请输入卡号');
      return;
    }
    
    try {
      const result = await validateCard(cardNumber);
      if (result.valid) {
        setValidationResult(`✅ 有效 - ${result.network} (${result.length}位)`);
      } else {
        setValidationResult(`❌ 无效 - ${result.reason}`);
      }
    } catch (error) {
      setValidationResult('❌ 验证失败');
    }
  };
  
  const handleGenerateUsers = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const users = await generateUsers(5);
      
      let result = '👥 虚假用户数据\n' + '='.repeat(50) + '\n\n';
      users.forEach((user, i) => {
        result += `👤 用户 #${i + 1}\n`;
        result += `🏷️  姓名: ${user.name}\n`;
        result += `📧 邮箱: ${user.email}\n`;
        result += `📱 电话: ${user.phone}\n`;
        result += `🏠 地址: ${user.address}\n`;
        result += '─'.repeat(40) + '\n';
      });
      
      setResult(result);
    } catch (error) {
      setError(error instanceof Error ? error.message : '生成失败');
    } finally {
      setLoading(false);
    }
  };
  
  const handleGenerateLorem = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const text = await generateLorem(loremCount);
      const result = `📝 Lorem Ipsum (${loremCount}段)\n${'='.repeat(50)}\n\n${text}`;
      
      setResult(result);
    } catch (error) {
      setError(error instanceof Error ? error.message : '生成失败');
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center gap-3 mb-6">
        <Search className="w-6 h-6 text-green-600" />
        <h2 className="text-2xl font-bold text-gray-800">工具</h2>
      </div>
      
      {/* 卡号验证工具 */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
          <Search className="w-5 h-5" />
          🔍 信用卡号码验证
        </h3>
        
        <div className="space-y-3">
          <div className="flex gap-3">
            <input
              type="text"
              value={cardNumber}
              onChange={(e) => setCardNumber(e.target.value)}
              placeholder="输入信用卡号码"
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent font-mono"
              onKeyPress={(e) => e.key === 'Enter' && handleValidate()}
            />
            <button
              onClick={handleValidate}
              className="bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-6 rounded-lg transition-colors duration-200"
            >
              验证
            </button>
          </div>
          
          {validationResult && (
            <div className={`p-3 rounded-lg ${
              validationResult.startsWith('✅') 
                ? 'bg-green-50 text-green-800' 
                : validationResult.startsWith('⚠️')
                ? 'bg-yellow-50 text-yellow-800'
                : 'bg-red-50 text-red-800'
            }`}>
              <p className="font-medium">{validationResult}</p>
            </div>
          )}
        </div>
      </div>
      
      {/* 虚假用户数据生成 */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
          <Users className="w-5 h-5" />
          👤 虚假用户数据生成
        </h3>
        
        <button
          onClick={handleGenerateUsers}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
        >
          <Users className="w-5 h-5" />
          🎲 生成用户数据
        </button>
      </div>
      
      {/* Lorem Ipsum 生成 */}
      <div className="bg-white rounded-lg border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
          <FileText className="w-5 h-5" />
          📝 Lorem Ipsum 生成
        </h3>
        
        <div className="space-y-3">
          <div className="flex items-center gap-3">
            <label className="text-sm font-medium text-gray-700">
              段落数:
            </label>
            <input
              type="number"
              min="1"
              max="10"
              value={loremCount}
              onChange={(e) => setLoremCount(parseInt(e.target.value) || 3)}
              className="w-20 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
            <button
              onClick={handleGenerateLorem}
              className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
            >
              <FileText className="w-5 h-5" />
              📄 生成文本
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}


