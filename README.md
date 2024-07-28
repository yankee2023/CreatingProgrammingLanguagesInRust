# CreatingProgrammingLanguagesInRust

## 目的

書籍「[Rustで作るプログラミング言語](https://www.amazon.co.jp/Rust%E3%81%A7%E4%BD%9C%E3%82%8B%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E-%E2%80%94%E2%80%94-%E3%82%B3%E3%83%B3%E3%83%91%E3%82%A4%E3%83%A9%EF%BC%8F%E3%82%A4%E3%83%B3%E3%82%BF%E3%83%97%E3%83%AA%E3%82%BF%E3%81%AE%E5%9F%BA%E7%A4%8E%E3%81%8B%E3%82%89%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E%E3%81%AE%E6%96%B0%E6%BD%AE%E6%B5%81%E3%81%BE%E3%81%A7-%E4%BD%90%E4%B9%85%E7%94%B0-%E6%98%8C%E5%8D%9A/dp/4297141922)」の修行。

## 目次

- [2.スタックベース仮想マシン](#2.スタックベース仮想マシン)

## 2.スタックベース仮想マシン

逆ポーランド記法、四則演算、優先演算、if制御構文

- unwrap  
エラーハンドリングではなく、プログラムの実行中に予期しないエラーが発生した場合に使用する。  
戻り値がNoneやErrの場合、'panic!'マクロが呼ばれクラッシュする。
