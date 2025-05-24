import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:invoy/src/rust/api/simple.dart';
import 'package:window_manager/window_manager.dart';

void Function() buildInvoiceAction({
  required InvoiceDirCubit invoiceDirCubit,
  required BuildInvoiceCubit buildInvoiceCubit,
}) =>
    () => buildInvoiceCubit.set(invoiceDirCubit.state);

void Function() pickInvoiceAction({required InvoicePromptDirCubit state}) =>
    () => state.pick();

class WindowMaximizedCubit extends Cubit<bool> {
  WindowMaximizedCubit() : super(false) {
    windowManager.addListener(MaximizationListener(maximizedState: this));
  }

  void set(bool isMaximized) async => emit(isMaximized);
}

class MaximizationListener extends WindowListener {
  MaximizationListener({required this.maximizedState});

  final WindowMaximizedCubit maximizedState;

  @override
  void onWindowMaximize() => maximizedState.set(true);

  @override
  void onWindowUnmaximize() => maximizedState.set(false);
}

class InvoiceDirCubit extends Cubit<String> {
  InvoiceDirCubit() : super("");

  void set(String invoiceDir) => emit(invoiceDir);
}

sealed class PickDirState {}

class PickDirDefault extends PickDirState {}

class PickDirBusy extends PickDirState {}

class PickDirCancelled extends PickDirState {}

class PickDirPicked extends PickDirState {
  PickDirPicked({required this.pickedDir});

  final String pickedDir;
}

class PickDir extends Cubit<PickDirState> {
  PickDir() : super(PickDirDefault());

  var _picking = false;

  Future<void> pick() async {
    if (_picking) {
      return;
    }
    _picking = true;

    emit(PickDirBusy());
    PickDirState state = PickDirCancelled();
    var pickedDir = await pickDir();
    _picking = false;

    if (pickedDir != null) {
      state = PickDirPicked(pickedDir: pickedDir);
    }

    print("emitting folder picking state");
    emit(state);
  }
}

class InvoicePromptDirCubit extends PickDir {}

class BuildInvoiceCubit extends Cubit<String?> {
  BuildInvoiceCubit() : super(null);

  var _building = false;

  Future<void> set(String invoiceDir) async {
    if (_building) {
      return;
    }

    _building = true;
    var err = await buildInvoice(inputDir: invoiceDir);
    _building = false;
    emit(err);
  }
}
