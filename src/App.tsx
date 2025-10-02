import { useState } from 'react';
import { CreditCard, Zap, Wrench } from 'lucide-react';
import { BasicMode } from './components/BasicMode';
import { AdvancedMode } from './components/AdvancedMode';
import { ToolsMode } from './components/ToolsMode';
import { ResultPanel } from './components/ResultPanel';

type Tab = 'basic' | 'advanced' | 'tools';

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('basic');
  
  return (
    <div className="h-screen bg-gradient-to-br from-blue-50 to-purple-50 p-6">
      <div className="max-w-7xl mx-auto h-full flex flex-col">
        {/* 标题区域 */}
        <header className="mb-6">
          <div className="bg-white rounded-lg shadow-lg p-6">
            <div className="flex items-center gap-3 mb-2">
              <CreditCard className="w-10 h-10 text-blue-600" />
              <div>
                <h1 className="text-3xl font-bold text-gray-800">
                  NamsoGen Desktop
                </h1>
                <p className="text-gray-600 text-sm mt-1">
                  专业的信用卡测试号码生成器 - 仅供开发测试使用
                </p>
              </div>
            </div>
            
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3 mt-4">
              <p className="text-yellow-800 text-sm">
                ⚠️ <strong>重要声明:</strong> 本工具生成的卡号仅用于测试目的，严禁用于非法用途！
              </p>
            </div>
          </div>
        </header>
        
        {/* 主内容区域 */}
        <div className="flex-1 grid grid-cols-5 gap-6 overflow-hidden">
          {/* 左侧控制面板 */}
          <div className="col-span-2 bg-white rounded-lg shadow-lg overflow-hidden flex flex-col">
            {/* 标签页导航 */}
            <div className="bg-gray-100 border-b border-gray-200">
              <div className="flex">
                <button
                  onClick={() => setActiveTab('basic')}
                  className={`flex-1 px-6 py-4 font-semibold transition-colors duration-200 flex items-center justify-center gap-2 ${
                    activeTab === 'basic'
                      ? 'bg-white text-blue-600 border-b-2 border-blue-600'
                      : 'text-gray-600 hover:bg-gray-200'
                  }`}
                >
                  <CreditCard className="w-5 h-5" />
                  🎯 基础模式
                </button>
                
                <button
                  onClick={() => setActiveTab('advanced')}
                  className={`flex-1 px-6 py-4 font-semibold transition-colors duration-200 flex items-center justify-center gap-2 ${
                    activeTab === 'advanced'
                      ? 'bg-white text-purple-600 border-b-2 border-purple-600'
                      : 'text-gray-600 hover:bg-gray-200'
                  }`}
                >
                  <Zap className="w-5 h-5" />
                  ⚡ 高级模式
                </button>
                
                <button
                  onClick={() => setActiveTab('tools')}
                  className={`flex-1 px-6 py-4 font-semibold transition-colors duration-200 flex items-center justify-center gap-2 ${
                    activeTab === 'tools'
                      ? 'bg-white text-green-600 border-b-2 border-green-600'
                      : 'text-gray-600 hover:bg-gray-200'
                  }`}
                >
                  <Wrench className="w-5 h-5" />
                  🔧 工具
                </button>
              </div>
            </div>
            
            {/* 标签页内容 */}
            <div className="flex-1 overflow-y-auto">
              {activeTab === 'basic' && <BasicMode />}
              {activeTab === 'advanced' && <AdvancedMode />}
              {activeTab === 'tools' && <ToolsMode />}
            </div>
          </div>
          
          {/* 右侧结果面板 */}
          <div className="col-span-3 shadow-lg">
            <ResultPanel />
          </div>
        </div>
        
        {/* 页脚 */}
        <footer className="mt-6 text-center text-gray-600 text-sm">
          <p>
            © 2025 NamsoGen Desktop | Powered by Tauri + Rust + React | 仅供合法测试使用 🔒
          </p>
        </footer>
      </div>
    </div>
  );
}

export default App;


