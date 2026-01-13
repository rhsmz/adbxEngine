/// コード補完の状態
#[derive(Default)]
pub struct CompletionState {
    pub is_visible: bool, // 補完候補が表示されているか
    pub candidates: Vec<CompletionCandidate>, // 補完候補のリスト
    pub selected_index: usize, // 選択されている候補のインデックス
    pub trigger_position: usize, // 補完がトリガーされた位置
    pub prefix: String, // 補完のプレフィックス（入力された文字列）
}

/// コード補完の候補
#[derive(Debug, Clone)]
pub struct CompletionCandidate {
    pub label: String, // 表示名
    pub insert_text: String, // 挿入するテキスト
    pub kind: CompletionKind, // 候補の種類
    pub detail: Option<String>, // 詳細情報（関数のシグネチャなど）
}

/// 補完候補の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Keyword,    // キーワード
    Function,   // 関数
    Variable,   // 変数
    Type,       // 型
    Property,   // プロパティ
    Method,     // メソッド
    Module,     // モジュール
}

/// 開いているファイルの情報
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub path: String,
    pub content: String,
    pub modified: bool,
}
