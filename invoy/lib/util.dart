import 'package:invoy/state.dart';
import 'package:window_manager/window_manager.dart';

void minimize() {
  windowManager.minimize();
}

Future<void> Function() unmaximize(WindowMaximizedCubit state) => () async {
  await windowManager.unmaximize();
  state.set(false);
};

Future<void> Function() maximize(WindowMaximizedCubit state) => () async {
  await windowManager.maximize();
  state.set(true);
};

void close() {
  windowManager.close();
}
