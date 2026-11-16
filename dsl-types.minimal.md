# DSL-Type minimal 草案 v2

以表达 rules.csv 为优先。本版把上次审计中的待补能力展开为字段和通用访问语义；尚未逐条证明覆盖全部 962 条，也未替换当前 operators.json 或 KLEE。

结构化定义及可解析示例见 [dsl-types.minimal.json](dsl-types.minimal.json)。字段表与此文件来自同一份定义。

## 身份与访问

- 基础类型为 Boolean、Integer、Float、String；Integer 是数学整数，无隐式机器溢出。Sequence/Set/Map 为逻辑容器，T? 表示可空。
- **State.objects:Map<String,Value>**；Value.identity 使用同一稳定字符串。State.allocations、resources、threads 及 Trace.invocations 同样用字符串键，Trace.states 用整数位置。
- 字符串身份包含种类/代次，不能只把地址转成字符串。已释放记录仍可按旧身份查询，新分配不得复用该身份。
- get_field(record,"name") 读取下表的建模属性；get_field(map,key) 按 String/Integer 键查找；get_field(sequence,index) 按整数索引，序列 length 也可读取。Map 迭代绑定 key，Sequence/Set 迭代绑定元素。
- get_field 统一表达真实 Rust 字段、tuple/enum payload、建模属性及容器成员投影。具体类别和同名消歧由解析器结合源声明/类型绑定识别，DSL 不增加专门算子或命名空间前缀。
- 上下文沿用 get_field(get_result(), "call") 的已有表示，解析器根据普通调用或 trait 契约绑定；trait 场景不要求构造真实返回值。普通 get_arg/get_result 的源参数/结果含义保持；enclosing 泛型参数排在方法泛型参数之前。
- 已知缺失的 Map key、越界 Sequence 索引及声明可空属性的已知缺失值返回 None；未声明字段或错误类型是定义错误，不能伪造字段。声明属性所需观测信息不足时返回 unknown，不能当成 true。

## 字段定义

Pointer 继承 Value。类型名称、种类专用属性和各容器类型的适用条件都要按定义检查，不允许凭字段名字猜测成功。

### Value

带源类型和观测状态的值；标量在表达式中透明表示其内容。种类专用属性仅在适用时可读。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| type | TypeInfo | 实际源类型；dyn 包装器的静态类型和被包装对象的具体类型分别保留 | rule-31, rule-784 |
| identity | String? | 稳定对象身份；无对象身份的逻辑标量为 None | rule-45, rule-845 |
| storage | MemoryRange? | 值自身的存储位置，不是指针指向的位置 | rule-39, rule-793 |
| call | CallContext | 绑定该值的上下文，unit 结果也适用 | rule-53, rule-852 |
| initialized | Boolean | 按源 Rust 类型完成初始化及语言有效性；不要求 padding 已写入 | rule-39, rule-53 |
| lifetime | Interval? | 借用、引用或 future 的语义存续区间 | rule-45, rule-62, rule-581 |
| length | Integer | 数组/slice 元素数，字符串字节数 | rule-41, rule-783 |
| element_type | TypeInfo | 序列元素或指针 pointee 的源类型 | rule-39, rule-804 |
| elements | Sequence<Value> | 源元素的有序视图；指针需先取 pointee | rule-38, rule-39 |
| bytes | Sequence<Value> | 对象表示字节，每项有数值、index 和字节 initialized 属性；允许未初始化 padding | rule-56, rule-787 |
| index | Integer | 元素在所属序列的索引 | rule-39 |
| variant | String | 当前枚举分支的源名称 | rule-79, rule-876 |
| discriminant | Integer | 当前枚举判别值，按源声明映射 | rule-79 |
| is_nan | Boolean | 浮点 NaN 分类，非浮点值不适用 | rule-202, rule-205, rule-208, rule-211 |
| is_infinite | Boolean | 浮点无穷分类，非浮点值不适用 | rule-203, rule-206, rule-209, rule-212 |
| truncated | Integer? | 有限浮点数向零截断的数学整数；NaN/无穷为 None | rule-201, rule-204, rule-207, rule-210 |
| created_at | Integer | 该身份对象的生命周期开始位置 | rule-845, rule-856 |
| dropped_at | Integer? | 该身份对象销毁完成位置；未销毁为 None | rule-227, rule-847 |
| observations | Sequence<Invocation> | 实际调用历史中以该对象为逻辑 receiver 的调用，不包含假设调用 | rule-99, rule-102, rule-859 |
| states | Map<Integer,Value> | 同一对象/值在各轨迹位置的观测视图，保留类型和指针来源；不存在时为 None | rule-30, rule-716 |
| descendants | Set<Value> | 反身传递的真实拥有字段/元素闭包；不沿非拥有引用追踪 | rule-227, rule-790, rule-794 |
| reachable | Set<Value> | 反身传递的字段、元素、指针及引用图可达闭包，按对象身份去重 | rule-197, rule-782, rule-901 |
| resource | Resource? | 描述符、handle 或 task 对应的资源状态；非资源或已知无效标识为 None | rule-713, rule-775, rule-888 |

### Pointer

Value 的指针子类；属性投影和数学字节偏移均保留来源与观测状态。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| address | Integer | 数值地址；逻辑偏移可产生超出机器地址空间的数学值 | rule-252, rule-268 |
| allocation | Allocation? | 派生来源的分配身份；释放后仍保留该记录，不能仅按当前地址重新匹配 | rule-3, rule-16 |
| length | Integer | slice-tail fat pointer 的长度 metadata，包括 struct-with-slice-tail；其他 pointer 种类不适用 | rule-11, rule-12 |
| element_type | TypeInfo | 源 pointee T 的类型；若 T 本身是 slice，其元素类型再由 T.element_type 读取 | rule-784, rule-804 |
| metadata | Value? | slice 长度标量或 trait-object vtable 指针的源元数据；thin pointer 为 None | rule-11, rule-14 |
| pointee | Value? | 可读取时的逻辑源对象；不隐含 Rust 库安全不变量 | rule-31, rule-38 |
| vtable | VTable? | trait-object 元数据的有类型视图；无法解析为编译器 vtable 记录时为 None；存在记录不隐含存储可读 | rule-14, rule-180, rule-188 |
| dynamic_size | Integer | 整个 DST 的字节数，含静态前缀及动态尾部；无可解析布局时 unknown | rule-12, rule-55 |
| dynamic_align | Integer | 整个动态 pointee 的对齐；无法解析时 unknown | rule-54 |
| derived_from | Set<String> | 真实借用/指针派生的传递祖先对象或借用身份 | rule-581 |
| borrow_identity | String? | 当前源引用/reborrow 的独立借用令牌，区别于 pointee 对象身份；不建立借用的 raw pointer 可为 None | rule-581 |
| nul_offset | Integer? | 同一来源活分配内、遇到未初始化字节之前的首个已初始化 NUL 距离，单位字节；无则 None | rule-63, rule-65 |
| wide_nul_offset | Integer? | 对应的首个已初始化零 16 位单元距离，单位为 16 位单元；无则 None | rule-880, rule-936 |

### Allocation

当前分配代次的记录；成功 grow/shrink 即使地址不变也产生新身份。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| identity | String | 带代次的稳定分配身份，不是地址字符串 | rule-3, rule-30 |
| start | Integer | 起始地址 | rule-3, rule-268 |
| size | Integer | 实际返回的非负字节数 | rule-16, rule-20 |
| minimum_size | Integer | Allocator fitting-layout 合约允许的最小大小，来自产生当前分配的请求 | rule-16, rule-20, rule-22, rule-25 |
| layout | Layout | 产生当前分配时请求的 layout；与实际返回 size 区分 | rule-2, rule-5 |
| allocator | Allocator | 具有契约身份的分配器 | rule-3, rule-17 |
| live | Boolean | 在相应观测状态下尚未失效 | rule-3, rule-30 |

### MemoryRange

携带来源和观测状态的半开字节范围；构造范围不建立可访问性。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| pointer | Pointer | 范围起点指针，保留 allocation 和观测状态 | rule-253, rule-268 |
| size | Integer | 非负字节长度 | rule-253, rule-268 |
| region | String | 整个数值区间在平台映射中为 user/enclave/other，跨类别为 mixed；不隐含分配包含关系或权限 | rule-806, rule-808 |

### Layout

源 Rust 分配布局的逻辑视图。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| size | Integer | 非负请求字节数 | rule-1, rule-19 |
| align | Integer | 请求对齐 | rule-2, rule-16 |

### Allocator

分配器的契约身份。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| id | String | 契约等价的复制/克隆分配器保留相同身份 | rule-3, rule-28 |

### TypeInfo

源类型及其编译器/契约元数据；集合是逻辑域，不要求枚举实现。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| id | String | 完整源类型身份，包含泛型实参；不能用显示名称代替 | rule-31 |
| name | String | 显示名称 | rule-31 |
| item_path | String | 不含泛型实参的规范定义路径，区分同名类型 | rule-791, rule-792 |
| lang_item | String? | 真实编译器 lang item；普通类型为 None | rule-790 |
| kind | String | primitive/struct/enum/array/slice/trait_object/extern/reference/raw_pointer/union 等源类别 | rule-11, rule-794 |
| size | Integer? | 静态大小，DST 为 None；无法取得信息则为 unknown | rule-59, rule-268 |
| align | Integer? | 静态对齐；无法静态确定为 None | rule-54, rule-59 |
| min | Integer? | 整数类型的最小表示值，其他类型为 None | rule-201 |
| max | Integer? | 整数类型的最大表示值，其他类型为 None | rule-201 |
| sized | Boolean | 是否满足源 Sized 约束 | rule-10, rule-175 |
| tail_type | TypeInfo? | 实际 DST 尾类型；整个类型为尾部时返回自身；无 DST 尾部为 None | rule-11, rule-14 |
| element_type | TypeInfo? | 数组/slice 元素类型，其他类型为 None | rule-39, rule-804 |
| pointee_type | TypeInfo? | 源引用、raw pointer 或有明确透明指针契约的 pointee 类型 | rule-784, rule-794 |
| fields | Map<String,TypeInfo> | 直接源字段类型，元组用十进制名称；数组元素另由 element_type 给出 | rule-790, rule-794 |
| traits | Set<String> | 实现的规范 trait 实例 ID，包含 trait 泛型实参 | rule-103, rule-113, rule-796 |
| trait_impls | Map<String,Implementation> | 按相同 trait 实例 ID 查对应实现；关联类型放在实现内，避免跨 trait 的同名冲突 | rule-103, rule-105 |
| methods | Map<String,MethodInfo> | 源公开及私有方法记录，key 为规范方法 ID；包含已解析的 trait/default 方法视图 | rule-105 |
| allows_uninitialized | Boolean | 该源类型能否合法地完全未初始化，区别于所有位模式有效 | rule-192 |
| has_padding | Boolean | 源布局中存在 padding 字节 | rule-58, rule-795 |
| has_provenance | Boolean | 该类型的值能承载指针 provenance，包括适用的嵌套字段/元素 | rule-57 |
| ownership | String | owning/borrowed/raw/none，依据源类型契约，不按名字猜测 | rule-790 |
| outlives | Set<String> | 源类型满足的生命周期边界名称，包括适用时的 static | rule-781, rule-947 |
| components | Set<TypeInfo> | 源拥有字段/元素类型的反身传递闭包，引用/raw pointer 的目标不是拥有组成部分 | rule-790, rule-791, rule-792, rule-794 |
| subtypes | Set<TypeInfo> | 源类型系统和 variance 确定的子类型域 | rule-113, rule-118 |
| supertypes | Set<TypeInfo> | 相应超类型域 | rule-113, rule-118 |
| bit_patterns | Set<Value> | 该静态大小的全部候选表示，值可不合法；候选的 provenance 信息按源模型保留 | rule-787 |
| valid_bit_patterns | Set<Value> | 其中满足 Rust 语言有效性的候选表示，不自动包含库安全不变量 | rule-787 |
| valid_values | Set<Value> | 满足语言有效性及该类型明确安全不变量的值域 | rule-56, rule-162, rule-198 |

### TraitInfo

规范的源 trait 实例。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| id | String | 带已绑定泛型实参的 trait 实例身份 | rule-103, rule-113 |
| item_path | String | trait 定义路径 | rule-103, rule-105 |
| type_arguments | Map<String,TypeInfo> | 源 trait 泛型实参，包括适用时的 Rhs | rule-56, rule-103 |
| methods | Map<String,MethodInfo> | trait 中声明的方法，按规范方法 ID 索引 | rule-73, rule-105 |

### Implementation

源 impl 记录；不预先编码其是否满足被验证规则。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| type | TypeInfo | 该实现的 Self 类型 | rule-105, rule-113 |
| trait | TraitInfo | 实现的 trait 实例 | rule-103, rule-105 |
| associated_types | Map<String,TypeInfo> | 按源名称解析的关联类型 | rule-103 |
| constants | Map<String,Value> | 按源名称解析的关联常量 | rule-195, rule-872 |
| methods | Map<String,MethodInfo> | 显式 impl 方法；不把继承的默认方法伪装为显式 override | rule-105 |

### MethodInfo

源方法及编译器已解析的 dispatch 信息。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| id | String | 规范方法实例身份 | rule-73, rule-105 |
| name | String | 不含路径的方法名 | rule-73 |
| qualified_name | String | 规范定义路径 | rule-105 |
| visibility | String | public/restricted/private，按源声明解释 | rule-105 |
| defining_trait | String? | 所属 trait 实例 ID；inherent 方法为 None | rule-105 |
| specialized | Boolean | 相对相应 generic/default 实现选择了非默认特化项的编译器事实；不等于整个规则正确 | rule-105, rule-108 |
| explicit_override | Boolean | 当前 impl 是否显式提供该方法体 | rule-124 |
| dispatch_targets | Set<MethodInfo> | 该入口在当前类型绑定下可到达的源 dispatch 项，含 helper 内的相关调用 | rule-105, rule-108 |
| invocations | Set<Invocation> | 该方法对所有允许输入和执行的逻辑调用域，不是仅实际发生的调用 | rule-56, rule-73 |

### VTable

由该指针元数据解析的编译器 vtable 观测，origin 属于此元数据的来源链。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| storage | MemoryRange | 编译器 vtable 记录所在的实际存储范围 | rule-14 |
| type | TypeInfo | vtable 对应的具体源类型 | rule-14 |
| target | TypeInfo | 对应的目标 dyn trait 类型，DST 包装类型应与 tail_type 比较 | rule-14 |
| trait | TraitInfo | 实际实现的 trait 实例 | rule-14 |
| origin | String | unsizing/other，依据该元数据实际产生及复制链；无证据则 unknown | rule-14 |
| methods | Map<String,MethodInfo> | 编译器发出的实际方法映射 | rule-14 |

### Interval

轨迹位置的半开区间 [start,end)。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| start | Integer | 起始状态位置 | rule-45, rule-852 |
| end | Integer? | 结束位置；None 表示尚未结束，不代表未来义务满足 | rule-45, rule-581 |

### Invocation

一次实际或契约域中的源调用；两类调用的域严格区分。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| identity | String | 稳定调用身份，trace 内唯一 | rule-852 |
| qualified_name | String | 实际源目标的规范路径；外部 primitive 使用平台操作表相同 key | rule-737, rule-852 |
| method | MethodInfo? | 目标源方法；非方法调用为 None | rule-73, rule-105 |
| args | Sequence<Value> | 按源顺序绑定的参数，receiver 在首位 | rule-19, rule-73 |
| result | Value? | 正常返回的源结果，unit 是 Value；尚未返回或无正常返回为 None | rule-53, rule-73 |
| result_type | TypeInfo | 声明/绑定后的源返回类型 | rule-201 |
| types | Map<String,TypeInfo> | 调用作用域内源类型名称及 Self、别名的绑定 | rule-103, rule-201 |
| consts | Map<String,Value> | 实际 const generic 绑定 | rule-195 |
| interval | Interval | 源调用进入至退出的区间 | rule-45, rule-852 |
| thread | String | 调用线程身份 | rule-769 |
| caller | String | 调用的逻辑拥有权主体/令牌身份 | rule-713, rule-775 |
| status | String | running/return/panic/unwind/abort/nontermination | rule-9, rule-53 |
| before | State | 进入前的快照 | rule-51, rule-852 |
| after | State? | 完成后的快照，执行尚未完成时为 None | rule-53 |
| trace | Trace | 包含此调用的执行轨迹；契约调用保留自身可能执行的轨迹 | rule-73, rule-805 |
| parent_invocation | String? | 真实因果 caller；信号/中断不因同线程而被当作普通 helper | rule-805 |
| stack | Sequence<Invocation> | 动态外层调用栈，不自动包含当前调用本身 | rule-925 |
| leaf_calls | Set<Invocation> | 其因果后代中的实际外部 primitive 调用，排除中间纯 Rust helper | rule-737 |

### Event

按 kind 区分的源/指令事件；可空字段由事件种类决定。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| identity | String | 稳定事件身份 | rule-255 |
| kind | String | create/read/write/move/drop/deallocate/clone/borrow_shared/borrow_mut/call/return/panic/unwind/atomic/fence/instruction/resource_open/resource_close/environment_read/environment_write/undefined_behavior | rule-45, rule-805 |
| time | Integer | 事件发生前的状态位置，事件后状态位于下一位置 | rule-45, rule-805 |
| thread | String | 实际线程身份 | rule-255 |
| invocation | String? | 直接归属的源调用身份，无调用归属的运行时事件为 None | rule-805 |
| causal_ancestors | Set<String> | 真实因果祖先调用身份，包含 helper 的外层调用，排除无关 signal/interrupt | rule-805 |
| object | String? | 受影响对象/借用的身份，不按地址相等替代 | rule-45, rule-845 |
| range | MemoryRange? | 实际访问范围，包含访问指针及来源 | rule-255, rule-581, rule-805 |
| value | Value? | 写入、创建或返回的源值 | rule-53 |
| atomic | Boolean | 该访问是否为 atomic | rule-255 |
| ordering | String? | 源 atomic ordering，无则 None | rule-763 |
| happens_before | Set<String> | 在该事件之前的真实 HB 事件身份，含规定的同步及传递关系，不是墙钟排序 | rule-255, rule-763 |
| unsafe_cell | Boolean | 访问字节是否位于 UnsafeCell | rule-49, rule-581 |
| lifetime | Interval? | 该借用事件建立的区间 | rule-45 |
| resource | Resource? | 受影响资源在本次事件状态下的观测 | rule-738 |
| unsafe_context | Boolean | 源码是否位于 unsafe 语法上下文，不表示操作正确 | rule-198 |
| instruction | String? | 实际机器指令的规范大写 mnemonic；无对应指令时为 None | rule-805 |
| strand | String | 线程内因果执行分支；普通 helper 继承，异步 handler 分支单独标识 | rule-805 |
| instruction_index | Integer? | 在该 strand 的机器指令位置；同条指令的访问事件使用相同位置 | rule-805 |
| state | State | 该事件 time 对应的源状态快照 | rule-255 |

### State

一个轨迹位置的源状态；记录和投影保留此状态。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| time | Integer | 轨迹位置 | rule-45 |
| objects | Map<String,Value> | 稳定对象身份字符串到该状态下对象的映射，key 与 Value.identity 相同 | rule-197, rule-845 |
| allocations | Map<String,Allocation> | 稳定分配身份字符串到记录的映射，包含可查询的已失效记录 | rule-3, rule-30 |
| resources | Map<String,Resource> | 平台资源状态，包括继承或外部创建的资源 | rule-713 |
| threads | Map<String,ThreadInfo> | 程序线程状态 | rule-773, rule-922 |
| globals | Map<String,Value> | 实际源全局变量 | rule-51 |
| environment | Map<String,Value> | 实际进程环境状态 | rule-738 |
| active_invocations | Set<String> | 此状态下仍活跃的调用身份 | rule-904 |
| phase | String | runtime_initialization/running/runtime_cleanup/thread_exit，由实际运行生命周期确定 | rule-852, rule-855, rule-923 |

### Trace

一次可能执行的轨迹；实际历史和契约的可能执行分别持有自己的 Trace。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| events | Sequence<Event> | 按观测位置排序的事件；排序本身不建立跨线程 HB | rule-45, rule-805 |
| invocations | Map<String,Invocation> | 按稳定调用身份查调用记录 | rule-852 |
| states | Map<Integer,State> | 按非负轨迹位置查状态 | rule-30, rule-716 |
| complete | Boolean | 是否已包含该执行的完整后续；未完成前缀不自动证明未来义务 | rule-45, rule-581 |

### Contract

与真实历史独立的完整逻辑域；禁止把待验证的规则假定为域前提。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| trait | TraitInfo? | 当前 trait 契约目标；普通函数契约可为 None | rule-73 |
| implementations | Set<Implementation> | 当前源约束允许的所有相关 impl，不限于运行中出现者 | rule-73, rule-105 |
| invocations | Set<Invocation> | 所有相关实现、允许输入及可能行为的调用域，每项有自己的 trace | rule-56, rule-73 |

### CallContext

调用/契约上下文；沿用 get_field(get_result(), "call") 的表示，由解析器按源场景绑定。trait 场景不要求真实返回值。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| invocation | Invocation? | 当前实际调用，trait 元契约无当前运行调用时为 None | rule-73, rule-852 |
| trace | Trace | 当前执行历史，不代替 Contract 的可能调用域 | rule-45, rule-852 |
| state | State | 当前求值状态，随时序求值前进 | rule-45, rule-716 |
| types | Map<String,TypeInfo> | 实际在作用域内的源名称/别名与 Self 绑定，不按规则臆造类型别名 | rule-103 |
| traits | Map<String,TraitInfo> | 源作用域内 trait 名称/别名到规范 trait 实例的绑定 | rule-103, rule-113 |
| consts | Map<String,Value> | 源 const generic 参数绑定 | rule-195 |
| constants | Map<String,Value> | 源作用域内普通常量绑定 | rule-195, rule-872 |
| contract | Contract? | 当前逻辑契约域；无则 None | rule-73 |
| platform | Platform | 目标平台及其独立操作契约 | rule-737, rule-805 |
| caller | String? | 当前调用的逻辑拥有权主体，trait 元契约入口可为 None | rule-713 |

### Resource

OS/平台资源事实，不要求 Rust owner 包装对象已经存在。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| identity | String | 稳定资源身份，区分关闭后的标识复用 | rule-713, rule-943 |
| kind | String | file_descriptor/socket/handle/task/event/module/process_environment/user_allocation 等资源类别 | rule-713, rule-775 |
| open | Boolean | 在该观测状态下的资源存续状态 | rule-713, rule-716 |
| owners | Set<String> | 具有对应处置/拥有权契约的主体令牌；不是 owner 包装器的存在集合 | rule-713, rule-775, rule-888 |
| closer | String | 足以释放该类资源的规范操作，如 close/closesocket/CloseHandle | rule-714, rule-749 |
| attributes | Map<String,Value> | 平台资源的实际属性；key 必须有源/平台定义，不允许添加每条规则的成功标志 | rule-755, rule-776, rule-877 |
| created_at | Integer | 该资源代次的创建位置 | rule-713 |
| closed_at | Integer? | 关闭位置；尚未关闭为 None | rule-716 |

### ThreadInfo

实际平台线程状态。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| identity | String | 线程身份 | rule-766, rule-941 |
| handle | Value | 实际平台线程 handle | rule-777, rule-889 |
| phase | String | starting/running/exit_destructors/exited | rule-922, rule-923 |
| tls | Map<String,Value> | 平台 TLS key 的规范字符串到当前值的映射 | rule-934 |
| stack | MemoryRange | 实际线程栈范围 | rule-779 |

### Platform

目标平台事实和外部原语契约；不提供每条 rule 的预判结果。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| name | String | 规范目标 OS/ABI 名称 | rule-702, rule-737 |
| regions | Sequence<AddressRegion> | 平台地址区域映射 | rule-806, rule-808 |
| operations | Map<String,Operation> | 按实际外部 primitive 规范名查平台契约，未知操作保持 unknown | rule-737 |

### AddressRegion

平台地址映射的一个区段。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| start | Integer | 半开区间起点 | rule-806 |
| end | Integer | 半开区间终点 | rule-808 |
| name | String | user/enclave/other 等实际区域类别 | rule-806, rule-808 |

### Operation

有独立平台规范依据的原语操作契约。

| Field | Type | 含义 | 规则示例 |
| --- | --- | --- | --- |
| qualified_name | String | 平台 primitive 的规范标识，与 Invocation.qualified_name 一致 | rule-737 |
| allowed_contexts | Set<String> | 平台规范允许的执行上下文，如 async_signal_safe；不能从一次成功调用推断 | rule-737 |
| attributes | Map<String,Value> | 平台契约其他明确属性；未定义 key 不产生任意符号真值 | rule-737, rule-841 |

## 求值与语义边界

1. &&、|| 和 => 按左侧决定是否读取右侧；false && X 为 false，true || X 为 true，false => X 为 true。对于 unknown，采用信息保守的三值逻辑：unknown && false 为 false，unknown || true 为 true，其余无法确定者保持 unknown。前提已知 false 时不求值不适用的分支。有效公式在某估值为 false 是违反，在全部需要的估值均为 true 才成立；其余为信息不足。未声明字段、错误类型等静态错误不能被短路掩盖。
2. forall/exists 量化声明的完整逻辑域。集合可能是符号表示的无限域；不能把有限观察子集当全集。未知域或无法完成证明时保持 unknown，不以空集合替代。Contract 的输入前提来自源类型和独立已知前提，不能包含当前被检验的结论。
3. Trace 只记录该次执行实际发生的历史；Contract.invocations/MethodInfo.invocations 是所有允许实现、输入和行为的域。每个假设 Invocation 有自己的 Trace。量化一个 Invocation 不自动切换外层上下文；需显式使用 inv.trace/inv.before/inv.result，避免误读当前实际调用。
4. 时序位置为当前 Trace 的状态位置。always 从当前位置起检查全部后续，eventually 检查当前或后续，previously 只检查较早位置，next_state 检查下一状态。未完成前缀不能证明尚未结束的未来义务；终止的完整有限轨迹无下一状态时 next_state 为 false。对假设调用的时序要求用该调用 trace.states 的显式量化，不能使用外层实际上下文替代。
5. 所有值、指针、范围及进一步投影保留观测状态。Value.states[t] 保留同一身份，并反映该位置的内存/资源状态；旧结果快照不能证明现在有效。Interval 使用 [start,end)，end=None 表示未结束；在不完整轨迹上对尚未结束区间的全程安全要求保留 unknown。
6. 事件按其 source object、range、来源指针和 causal_ancestors 关联；同线程/同时间不等于同一因果调用。借用图和拥有图边不同，closures 按各字段定义去重。unsafe_context 只描述语法环境。instruction_index 只用于相同 thread/strand 的指令次序，不替代跨线程 HB。
7. TypeInfo.traits 和 trait_impls 使用同一个规范 trait 实例 ID，含泛型实参；关联类型放在相应 Implementation 中。同名方法靠规范 id/defining_trait 区分。public wrapper 的实际特化应沿 dispatch_targets 检查相应 spec 项，不能把 wrapper 自身没有特化体判成违反。
8. 指针的 type 是静态源指针类型；其 pointee 保留真实源对象的类型。dyn 包装器的具体内容由其 vtable.type 与 pointee 的动态对象视图关联，不能把 dyn 的静态 TypeInfo 当作具体内容类型。VTable.origin 描述这份 metadata 的实际来源链，不能只凭地址落到某个 vtable 就认定是 unsizing。
9. bit_patterns 是候选表示的逻辑域，可含无效值；valid_bit_patterns 只含语言有效者，valid_values 还要求明确的类型安全不变量。候选不合法时不能任意读取其源字段。has_provenance 描述类型承载指针来源的能力，不描述某个具体指针是否存活。
10. is_valid_for(range,"initialized",T) 与 is_initialized_for_type(range,T) 统一采用类型初始化/语言有效性，允许 padding 未写入。所有字节均已写入是独立性质，不能用该谓词替代。已有 read/write、borrow、drop、encoding 等模式保留各自明确的范围、类型和时间语义，不相互隐含。
11. MemoryRange.region 只按 Platform.regions 判断整个数学区间；跨类别为 mixed，无匹配为 other，零长度按起点分类。它不建立 allocation 包含、初始化、权限或非空条件；源规则中的零大小例外仍须显式表达。
12. Resource.owners 是实际处置权主体集合，能表达 caller 的拥有权；不从 OwnedFd 等包装器是否存在推断。只有规则明确要求 exclusive 时才要求排除其他拥有者。Operation.allowed_contexts 来自独立平台契约；未知原语不默认允许。

13. DSL 的集合成员关系和比较是模型中的逻辑比较，不隐式调用用户实现的 PartialEq。对象/资源句柄按稳定身份比较；候选表示按类型化表示及适用的 provenance 比较。若规则讨论源 eq/ne 方法的行为，应显式量化对应方法调用及结果。

## 可以等价展开的便捷字段

| 字段 | 使用现有 DSL 的展开 |
| --- | --- |
| Pointer.is_null | address == 0 |
| Pointer.offset | allocation 存在时 address - allocation.start，允许负数 |
| Allocation.end | start + size |
| MemoryRange.start/end/allocation | pointer.address / pointer.address + size / pointer.allocation |
| TypeInfo.is_zst | 静态 size 已知时 size == 0 |

minimum_size 本版显式保留；它与实际分配 size 不同。length、闭包、NUL 位置等保留了正式访问定义，不以“后端能计算”为由删除。

## 表达式检查样例

完整/片段标记只区分样例的范围，不表示已经做了全部语义证明。结构化文件中保留全部 20 个规则探针及接口检查。

### rule-16

`layout` must fit that block of memory.

```text
get_field(get_arg(1), "allocation") != None && get_field(get_field(get_arg(1), "allocation"), "live") && get_field(get_arg(2), "align") == get_field(get_field(get_field(get_arg(1), "allocation"), "layout"), "align") && get_field(get_arg(2), "size") >= get_field(get_field(get_arg(1), "allocation"), "minimum_size") && get_field(get_arg(2), "size") <= get_field(get_field(get_arg(1), "allocation"), "size")
```

### rule-38

Every byte in the array must be in `0..=127`.

```text
forall byte in get_field(get_field(get_arg(0), "pointee"), "elements"): byte >= 0 && byte <= 127
```

### rule-58

Self and Rhs have no padding.

```text
!get_field(get_field(get_field(get_field(get_result(), "call"), "types"), "Self"), "has_padding") && !get_field(get_field(get_field(get_field(get_result(), "call"), "types"), "Rhs"), "has_padding")
```

### rule-73

`digit` must return an ASCII character.

```text
forall inv in get_field(get_field(get_field(get_result(), "call"), "contract"), "invocations"): (get_field(inv, "method") != None && get_field(get_field(inv, "method"), "name") == "digit" && get_field(get_field(inv, "method"), "defining_trait") == get_field(get_field(get_field(get_field(get_result(), "call"), "contract"), "trait"), "id") && get_field(inv, "status") == "return") => (get_field(inv, "result") >= 0 && get_field(inv, "result") <= 127)
```

### rule-201

The value must be representable in the return type `Int`, after truncating off its fractional part.

```text
get_field(get_arg(0), "truncated") != None && get_field(get_arg(0), "truncated") >= get_field(get_type_at_func(0), "min") && get_field(get_arg(0), "truncated") <= get_field(get_type_at_func(0), "max")
```

### rule-713

The resource pointed to by `fd` must be open and suitable for assuming ownership.

```text
get_field(get_arg(0), "resource") != None && get_field(get_field(get_arg(0), "resource"), "open") && get_field(get_field(get_result(), "call"), "caller") in get_field(get_field(get_arg(0), "resource"), "owners")
```

cleanup 操作要求属于 rule-714，不混入此公式；owners 是处置权主体令牌，不要求 Rust owner 包装器存在或额外要求唯一性。

### rule-787

Any bit pattern must be valid for the type.

```text
forall bits in get_field(get_field(get_field(get_field(get_result(), "call"), "types"), "Self"), "bit_patterns"): bits in get_field(get_field(get_field(get_field(get_result(), "call"), "types"), "Self"), "valid_bit_patterns")
```

### rule-852

This function must be called only once during runtime initialization.

```text
get_field(get_field(get_field(get_result(), "call"), "state"), "phase") == "runtime_initialization" && (forall key in get_field(get_field(get_field(get_result(), "call"), "trace"), "invocations"): (get_field(get_field(get_field(get_field(get_field(get_result(), "call"), "trace"), "invocations"), key), "qualified_name") == get_field(get_field(get_field(get_result(), "call"), "invocation"), "qualified_name") && get_field(get_field(get_field(get_field(get_field(get_field(get_result(), "call"), "trace"), "invocations"), key), "before"), "phase") == "runtime_initialization") => get_field(get_field(get_field(get_field(get_field(get_result(), "call"), "trace"), "invocations"), key), "identity") == get_field(get_field(get_field(get_result(), "call"), "invocation"), "identity"))
```

在当前调用进入状态上检查历史，trace 中未发生的未来调用不参与本次调用前置条件。

## 尚需明确的原始契约

- rule-125 的 cheap 没有明确成本度量或阈值。不能擅自等同于 O(1)，也不加一个 is_cheap 成功标志；精确性部分和成本部分分别记录。
- rule-737 的平台原语允许上下文已具备表达接口，但具体平台操作表仍必须依据目标平台规范填写，不能凭调用名猜测。
- rule-841/842 等 ABI 引用仍需绑定具体外部契约；本版不会把缺少契约解释的条款写成 true。
- rule-14、105、805 的 JSON 样例明确标为片段或候选，仍需结合源上下文/平台契约完成完整翻译。本版补齐其所需的数据及通用访问能力，不声称这三条已语义验证。

本版只交付语言设计草案及表达路径检查结果；解析器/KLEE 的实际接入属于后续实现。
