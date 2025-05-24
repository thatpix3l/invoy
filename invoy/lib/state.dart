import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:invoy/src/rust/api/simple.dart';
import 'package:window_manager/window_manager.dart';

class InvoiceDirCubit extends Cubit<String?> {
  InvoiceDirCubit() : super(null);

  void set(String invoiceDir) => emit(invoiceDir);
}

void Function() buildInvoiceAction(BuildContext context, String? invoiceDir) =>
    () {
      if (invoiceDir == null) {
        return;
      }

      context.read<BuildInvoiceCubit>().set(invoiceDir);
    };

void Function() pickInvoiceAction(BuildContext context) =>
    () => context.read<PickInvoiceDirCubit>().set(
      context.read<InvoiceDirCubit>(),
    );

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

class PickInvoiceDirCubit extends Cubit<Null> {
  PickInvoiceDirCubit() : super(null);

  var picking = false;

  Future<void> set(InvoiceDirCubit invoiceDirCubit) async {
    if (picking) {
      return;
    }

    picking = true;
    var pickedInvoiceDir = await pickInvoiceDir();
    picking = false;

    if (pickedInvoiceDir != null) {
      invoiceDirCubit.set(pickedInvoiceDir);
      emit(null);
    }
  }
}

class BuildInvoiceCubit extends Cubit<String?> {
  BuildInvoiceCubit() : super(null);

  var building = false;

  Future<void> set(String invoiceDir) async {
    if (building) {
      return;
    }

    building = true;
    var err = await buildInvoice(inputDir: invoiceDir);
    building = false;
    emit(await buildInvoice(inputDir: invoiceDir));
  }
}
