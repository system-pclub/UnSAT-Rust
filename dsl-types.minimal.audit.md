# DSL-Type minimal v2 检查结果

本版已把上次审计中的字段和通用访问缺项展开为定义，并按要求将 State.objects 改为 Map<String,Value>。字段路径检查通过；这不等于全部规则的语义证明。

## 实际检查

- 在 unsat-rust:local、linux/amd64 容器中运行，仓库以只读方式挂载。
- 字段类型引用闭合：23 个类型、189 个显式字段；包括 Pointer 对 Value 的同类型属性覆盖。
- 20 个规则探针和 4 个接口表达式均能使用现有的 16 个算子解析，所需声明路径、Map 键类型及量词变量绑定均可解析。
- 5 个反向检查被拒绝：未定义的正确性标志、State.objects 的整数键、自由变量、错误算子参数个数、臆造的源字段。
- 未新增算子，草案算子名称集合与当前 operators.json 完全一致。源字段、建模属性及上下文绑定由解析器识别；这里只定义表达所需信息，没有改动现有 parser、operators.json、ptr_rule_dsl.json 或 KLEE。
- 结构化定义快照 SHA-256：4fd4791ca1aee3d7bc7ccd9f96267e42bc20344ee5867c3c3878749bb47e4b11。

## 上次缺项的处理

| 原缺口 | 本版定义 |
| --- | --- |
| 对象身份键与 get_field 不兼容 | Value.identity 为 String?，State.objects 为 Map<String,Value>；其他身份索引同样使用稳定 String |
| 动态布局/vtable 只有需求说明 | Pointer.length/dynamic_size/dynamic_align/vtable 与完整 VTable 字段 |
| padding/provenance/未初始化能力 | TypeInfo.has_padding/has_provenance/allows_uninitialized |
| 浮点截断和整数上下界 | Value.is_nan/is_infinite/truncated 与 TypeInfo.min/max |
| trait 输入及实现域缺失 | 独立 Contract、TraitInfo、Implementation、MethodInfo；契约域与实际 Trace 分开 |
| 方法可见性及特化 | MethodInfo.visibility/specialized/explicit_override/dispatch_targets |
| 资源状态与拥有权 | Value.resource、Resource、CallContext.caller；资源记录不依赖 owner 包装器存在 |
| 指令及运行阶段 | Event.instruction/strand/instruction_index/causal_ancestors，State.phase |
| 源字段与元属性同名 | 统一用 get_field，具体类别和消歧由解析器结合源声明、类型与上下文识别 |
| nullable/unknown/历史快照无约定 | 明确缺失值、定义错误、三值逻辑和短路、时序与观测状态语义 |
| 派生/借用身份混同 | Pointer.borrow_identity 与 pointee.identity 分开，derived_from 可包含稳定借用令牌 |

## 规则探针

| 规则 | 声明路径及查找键 | 样例范围 |
| --- | --- | --- |
| rule-1 | 已解析 | 完整候选式，尚未语义证明 |
| rule-3 | 已解析 | 完整候选式，尚未语义证明 |
| rule-16 | 已解析 | 完整候选式，尚未语义证明 |
| rule-19 | 已解析 | 完整候选式，尚未语义证明 |
| rule-38 | 已解析 | 完整候选式，尚未语义证明 |
| rule-268 | 已解析 | 完整候选式，尚未语义证明 |
| rule-12 | 已解析 | 完整候选式，尚未语义证明 |
| rule-14 | 已解析 | 片段/候选路径，需完成完整翻译 |
| rule-57 | 已解析 | 完整候选式，尚未语义证明 |
| rule-58 | 已解析 | 完整候选式，尚未语义证明 |
| rule-73 | 已解析 | 完整候选式，尚未语义证明 |
| rule-103 | 已解析 | 完整候选式，尚未语义证明 |
| rule-105 | 已解析 | 片段/候选路径，需完成完整翻译 |
| rule-113 | 已解析 | 完整候选式，尚未语义证明 |
| rule-192 | 已解析 | 完整候选式，尚未语义证明 |
| rule-201 | 已解析 | 完整候选式，尚未语义证明 |
| rule-713 | 已解析 | 完整候选式，尚未语义证明 |
| rule-787 | 已解析 | 完整候选式，尚未语义证明 |
| rule-805 | 已解析 | 片段/候选路径，需完成完整翻译 |
| rule-852 | 已解析 | 完整候选式，尚未语义证明 |

完整表达式、源参数类型和每条样例的说明保存在 dsl-types.minimal.json。

## 保留的验证边界

- 没有逐条翻译和验证 rules.csv 的全部 962 条，不给出全量覆盖率。
- 路径检查器不执行完整的可空值流分析、动态源类型/种类适用性检查、标量类型检查、程序行为或时序证明，也不执行 KLEE。源字段检查样例提供已解析的源绑定；检查器不实现或验证实际解析器的自动消歧。
- rule-14 的样例只检查 vtable 类型/来源路径；完整条件还需元数据初始化及存储有效性。
- rule-105 的样例能访问实际 public 方法和 spec dispatch 特化事实，但完整公式仍需保留 setup/特化分支的源条件，避免误判 generic/default 实现。
- rule-805 的候选式包含逐写入、对齐例外、因果范围与相邻屏障序列；instruction_index/strand 的平台观测仍需与 Intel 契约核对。
- cheap 的成本标准、平台 primitive 的允许上下文表、外部 ABI 引用仍需独立契约定义；没有用每条规则的成功布尔值代替。

## 交付文件

- dsl-types.minimal.md：字段及语义说明，供审阅。
- dsl-types.minimal.json：同一份结构化类型定义、16 个已有算子及 24 个表达式检查样例。
