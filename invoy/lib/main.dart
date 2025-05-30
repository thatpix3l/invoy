import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_material_design_icons/flutter_material_design_icons.dart';
import 'package:invoy/src/rust/frb_generated.dart';
import 'package:invoy/state.dart';
import 'package:invoy/util.dart';

class ColorPalette {
  static const bright = Color.fromARGB(255, 255, 247, 241);
  static const mediumBright = Color.fromARGB(255, 165, 158, 154);
  static const mediumDark = Color.fromARGB(255, 46, 44, 43);
  static const dark = Color.fromARGB(255, 0, 0, 0);

  static const accent = Color.fromARGB(255, 255, 110, 42);
  static const accentBright = Color.fromARGB(255, 255, 176, 140);
  static const accentDark = Color.fromARGB(255, 114, 36, 0);
}

Future<void> main(List<String> args) async {
  await RustLib.init();

  // Run app
  runApp(const MyApp());
}

Future<void> Function() maximizeStatusAction(
  bool isMaximized,
  WindowMaximizedCubit state,
) => isMaximized ? unmaximize(state) : maximize(state);

IconData maximizeStatusIcon(bool isMaximized) =>
    isMaximized ? Icons.filter_none_outlined : Icons.crop_square_outlined;

class SidebarActionTooltip extends StatelessWidget {
  const SidebarActionTooltip({this.child, required this.message, super.key});

  final String message;
  final Widget? child;

  @override
  Widget build(BuildContext context) => Tooltip(
    message: message,
    showDuration: Duration(seconds: 10),
    waitDuration: Duration(milliseconds: 500),
    decoration: BoxDecoration(
      color: ColorPalette.dark,
      borderRadius: const BorderRadius.all(Radius.circular(4)),
    ),
    textStyle: TextStyle(color: ColorPalette.bright),
    verticalOffset: -13,
    margin: EdgeInsets.only(left: 70),
    child: child,
  );
}

class PickInvoiceButton extends StatelessWidget {
  const PickInvoiceButton({super.key});

  @override
  Widget build(BuildContext context) => FloatingActionButton(
    onPressed: pickInvoiceAction(state: context.read()),
    hoverColor: ColorPalette.dark.withAlpha(100),
    splashColor: ColorPalette.accent.withAlpha(150),
    backgroundColor: ColorPalette.mediumDark.withAlpha(200),
    foregroundColor: ColorPalette.bright,
    child: BlocBuilder<InvoicePromptDirCubit, PickDirState>(
      builder: (context, state) => switch (state) {
        PickDirBusy() => CircularProgressIndicator(color: ColorPalette.bright),
        PickDirPicked() => () {
          context.read<InvoiceDirCubit>().set(state.pickedDir);
          return Icon(Icons.folder);
        }(),
        _ => Icon(Icons.folder),
      },
    ),
  );
}

class PickInvoiceSidebarButton extends StatelessWidget {
  const PickInvoiceSidebarButton({super.key});

  @override
  Widget build(BuildContext context) => const SidebarActionTooltip(
    message: "Pick Invoice Directory",
    child: PickInvoiceButton(),
  );
}

class BuildInvoiceButton extends StatelessWidget {
  const BuildInvoiceButton({super.key});

  @override
  Widget build(BuildContext context) => FloatingActionButton(
    onPressed: buildInvoiceAction(
      buildInvoiceCubit: context.read(),
      invoiceDirCubit: context.read(),
    ),
    hoverColor: ColorPalette.dark.withAlpha(100),
    splashColor: ColorPalette.accent.withAlpha(150),
    backgroundColor: ColorPalette.mediumDark.withAlpha(200),
    foregroundColor: ColorPalette.bright,
    child: const Icon(MdiIcons.hammer),
  );
}

class BuildInvoiceSidebarButton extends StatelessWidget {
  const BuildInvoiceSidebarButton({super.key});

  @override
  Widget build(BuildContext context) => const SidebarActionTooltip(
    message: "BuildInvoice",
    child: BuildInvoiceButton(),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) => MultiBlocProvider(
    providers: [
      BlocProvider(create: (BuildContext context) => BuildInvoiceCubit()),
      BlocProvider(create: (BuildContext context) => InvoiceDirCubit()),
      BlocProvider(create: (BuildContext context) => InvoicePromptDirCubit()),
      BlocProvider(create: (BuildContext context) => WindowMaximizedCubit()),
    ],
    child: MaterialApp(
      home: Scaffold(
        backgroundColor: ColorPalette.bright,
        appBar: AppBar(
          title: const Text('Invoy'),
          backgroundColor: ColorPalette.bright,
          actionsPadding: EdgeInsets.all(8),
          actions: [
            const IconButton(onPressed: minimize, icon: Icon(Icons.minimize)),
            BlocBuilder<WindowMaximizedCubit, bool>(
              builder: (context, isMaximized) => IconButton(
                onPressed: maximizeStatusAction(isMaximized, context.read()),
                icon: Icon(maximizeStatusIcon(isMaximized)),
              ),
            ),
            IconButton(onPressed: close, icon: Icon(Icons.clear)),
          ],
        ),
        body: Row(
          spacing: 16,
          children: [
            const Padding(
              padding: EdgeInsetsGeometry.all(16),
              child:
                  // Sidebar actions for manipulating an invoice.
                  Column(
                    spacing: 16,
                    children: [
                      PickInvoiceSidebarButton(),
                      BuildInvoiceSidebarButton(),
                    ],
                  ),
            ),
            Expanded(
              child: Align(
                alignment: Alignment.bottomRight,
                child: Column(
                  children: [
                    // Expanded(child: Spacer()),
                    // Expanded(
                    //   child: Text(
                    //     'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
                    //   ),
                    // ),
                    // TextFormField(
                    //   initialValue: 'Input text',
                    //   decoration: InputDecoration(
                    //     labelText: 'Label text',
                    //     errorText: 'Error message',
                    //     border: OutlineInputBorder(),
                    //     suffixIcon: Icon(Icons.error),
                    //   ),
                    // ),
                    // Expanded(child: Spacer()),
                  ],
                ),
              ),
            ),

            // Expanded(child: Spacer()),
          ],
        ),
      ),
    ),
  );
}
