import 'package:flutter/material.dart';
import 'package:flutter_material_design_icons/flutter_material_design_icons.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:invoy/src/rust/api/simple.dart';
import 'package:invoy/src/rust/frb_generated.dart';
import 'package:window_manager/window_manager.dart';

class ColorPalette {
  static const bright = Color.fromARGB(255, 255, 247, 241);
  static const mediumBright = Color.fromARGB(255, 165, 158, 154);
  static const mediumDark = Color.fromARGB(255, 46, 44, 43);
  static const dark = Color.fromARGB(255, 0, 0, 0);

  static const accent = Color.fromARGB(255, 255, 110, 42);
  static const accentBright = Color.fromARGB(255, 255, 176, 140);
  static const accentDark = Color.fromARGB(255, 114, 36, 0);
}

final isMaximizedProvider = FutureProvider.autoDispose((ref) async {
  return await windowManager.isMaximized();
});

String? pickedInvoiceDir;

final pickedInvoiceDirProvider = FutureProvider.autoDispose((ref) async {
  pickedInvoiceDir = await pickInvoiceDir();
});

Future<void> main(List<String> args) async {
  await RustLib.init();

  // Run app
  runApp(ProviderScope(child: const MyApp()));
}

void minimize() {
  windowManager.minimize();
}

void unmaximize() {
  windowManager.unmaximize();
}

void maximize() {
  windowManager.maximize();
}

void close() {
  windowManager.close();
}

class ToggleWindowData {
  late void Function() action;
  late IconData iconData;

  ToggleWindowData(bool isMaximized) {
    if (isMaximized) {
      action = unmaximize;
      iconData = MdiIcons.squareMediumOutline;
    } else {
      action = maximize;
      iconData = Icons.square_outlined;
    }
  }
}

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

Widget pickInvoiceDirWidgetBuilder(
  BuildContext context,
  WidgetRef ref,
  Widget? child,
) => FloatingActionButton(
  onPressed: () => ref.refresh(pickedInvoiceDirProvider),
  hoverColor: ColorPalette.dark.withAlpha(100),
  splashColor: ColorPalette.accent.withAlpha(150),
  backgroundColor: ColorPalette.mediumDark.withAlpha(200),
  foregroundColor: ColorPalette.bright,
  child: const Icon(Icons.folder),
);

const (String, Widget) pickInvoiceDirRecord = (
  "Pick Invoice Directory",
  Consumer(builder: pickInvoiceDirWidgetBuilder),
);

String? buildInvoicePreviousRunError;

void maybeBuildInvoice() async {
  buildInvoicePreviousRunError = await buildInvoice(inputDir: pickedInvoiceDir);
}

(String, Widget) buildInvoiceRecord = (
  "Build Invoice",
  FloatingActionButton(
    onPressed: maybeBuildInvoice,
    hoverColor: ColorPalette.dark.withAlpha(100),
    splashColor: ColorPalette.accent.withAlpha(150),
    backgroundColor: ColorPalette.mediumDark.withAlpha(200),
    foregroundColor: ColorPalette.bright,
    child: const Icon(MdiIcons.hammer),
  ),
);

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        backgroundColor: ColorPalette.bright,
        appBar: AppBar(
          title: const Text('Invoy'),
          backgroundColor: ColorPalette.bright,
          actionsPadding: EdgeInsets.all(8),
          actions: [
            IconButton(onPressed: minimize, icon: Icon(Icons.minimize)),
            Consumer(
              builder: (context, ref, child) {
                final isMaximizedValue = ref.watch(isMaximizedProvider);

                var ToggleWindowData(
                  :action,
                  :iconData,
                ) = switch (isMaximizedValue) {
                  AsyncData(:final value) => ToggleWindowData(value),
                  _ => ToggleWindowData(true),
                };

                ref.invalidate(isMaximizedProvider);

                return IconButton(onPressed: action, icon: Icon(iconData));
              },
            ),
            // IconButton(onPressed: maximize, icon: Icon(Icons.square_outlined)),
            IconButton(onPressed: close, icon: Icon(Icons.clear)),
          ],
        ),
        body: Row(
          children: [
            Container(
              // color: ColorPalette.bright,
              child: Padding(
                padding: EdgeInsetsGeometry.all(16),
                child:
                    // Sidebar actions for manipulating an invoice.
                    Column(
                      spacing: 16,
                      children: [pickInvoiceDirRecord, buildInvoiceRecord]
                          .map(
                            (record) => SidebarActionTooltip(
                              message: record.$1,
                              child: record.$2,
                            ),
                          )
                          .toList(),
                    ),
              ),
            ),
            Center(
              child: Container(
                // color: Color.fromARGB(255, 222, 233, 255),
                child: Padding(
                  padding: EdgeInsets.symmetric(
                    horizontal: 16.0,
                    vertical: 16.0,
                  ),
                  child: Column(
                    children: [
                      Expanded(child: Spacer()),
                      Text(
                        'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
                      ),
                      // TextFormField(
                      //   initialValue: 'Input text',
                      //   decoration: InputDecoration(
                      //     labelText: 'Label text',
                      //     errorText: 'Error message',
                      //     border: OutlineInputBorder(),
                      //     suffixIcon: Icon(Icons.error),
                      //   ),
                      // ),
                      Expanded(child: Spacer()),
                    ],
                  ),
                ),
              ),
            ),
            Expanded(child: Spacer()),
          ],
        ),
      ),
    );
  }
}
