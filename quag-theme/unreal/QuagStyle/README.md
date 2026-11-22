# QuagStyle: house style for my Unreal Editor plugins

Slate version of the Quag style (APP-UI's `DESIGN.md`, §9 has the rules).
Targets **UE 5.3+** (it uses `FStyleColors`, `FAppStyle`, `FSlateRoundedBoxBrush`, and `FVector2f` brush sizes).

## Install

1. `node <APP-UI>/tools/quag.mjs unreal <Project>` copies this folder into `<Project>/Plugins/QuagStyle`, without
   `Resources/Mascot/` (add `--mascot` for a private plugin). The project owns that copy.
2. In your plugin's `.uplugin`:
   ```json
   "Plugins": [ { "Name": "QuagStyle", "Enabled": true } ]
   ```
3. In your module's `Build.cs`: `PrivateDependencyModuleNames.Add("QuagStyle");`

### Public plugins (Fab / shared with others)
- **Delete `Resources/Mascot/`.** The mascot then switches itself off (`HasMascot()` returns false and
  `SQuagMascot` renders nothing), so no code changes are needed. The GIF belongs to its artist and can't ship.
- Fab only accepts self-contained plugins. Copy `QuagStyle.h/.cpp` into your plugin's own module,
  rename the class (e.g. `FMyPluginStyle`), the style set name and the `QUAGSTYLE_API` macro, and drop the mascot files.

## Use

```cpp
#include "QuagStyle.h"
#define LOCTEXT_NAMESPACE "AssetAuditor"

TSharedRef<SDockTab> FAssetAuditorModule::SpawnTab(const FSpawnTabArgs&)
{
	return SNew(SDockTab).TabRole(ETabRole::NomadTab)
	[
		SNew(SVerticalBox)

		// Header: brand title left, ONE primary action right
		+ SVerticalBox::Slot().AutoHeight().Padding(12.f, 10.f)
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(1.f).VAlign(VAlign_Center)
			[
				FQuagStyle::MakeTitle(LOCTEXT("Title", "Asset Auditor"), LOCTEXT("Sub", "42 assets"))
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(4.f, 0.f)
			[
				FQuagStyle::MakeButton(LOCTEXT("Export", "Export CSV"), EQuagButton::Secondary,
					FSimpleDelegate::CreateRaw(this, &FAssetAuditorModule::Export))
			]
			+ SHorizontalBox::Slot().AutoWidth()
			[
				FQuagStyle::MakeButton(LOCTEXT("Scan", "Scan project"), EQuagButton::Primary,
					FSimpleDelegate::CreateRaw(this, &FAssetAuditorModule::Scan))
			]
		]

		// Body: card around the list (Quag outline = this is where focus is)
		+ SVerticalBox::Slot().FillHeight(1.f).Padding(12.f, 0.f)
		[
			FQuagStyle::MakeCard(
				Issues.Num() == 0
					? FQuagStyle::MakeEmptyState(LOCTEXT("Empty", "No issues found."),
					                             LOCTEXT("Hint", "Run a scan to check the project."))
					: StaticCastSharedRef<SWidget>(MakeIssueList()),
				/*bFocused*/ true)
		]

		// Status line
		+ SVerticalBox::Slot().AutoHeight().Padding(12.f, 8.f)
		[
			FQuagStyle::MakeStatus(EQuagStatus::Ok, LOCTEXT("Done", "Scan complete · 2s ago"))
		]
	];
}

// List rows: Quag selection + tags + status
TSharedRef<ITableRow> FAssetAuditorModule::MakeRow(TSharedPtr<FIssue> Item, const TSharedRef<STableViewBase>& Owner)
{
	return SNew(STableRow<TSharedPtr<FIssue>>, Owner)
		.Style(&FQuagStyle::Get().GetWidgetStyle<FTableRowStyle>("Quag.TableRow"))
		.Padding(FMargin(8.f, 4.f))
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(1.f).VAlign(VAlign_Center)
			[
				SNew(STextBlock).TextStyle(&FQuagStyle::Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text"))
				.Text(Item->Name)
			]
			+ SHorizontalBox::Slot().AutoWidth().Padding(8.f, 0.f)
			[
				FQuagStyle::MakeTag(Item->Category)
			]
			+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center)
			[
				FQuagStyle::MakeStatus(Item->bError ? EQuagStatus::Err : EQuagStatus::Warn, Item->Short)
			]
		];
}
#undef LOCTEXT_NAMESPACE
```

## What's in the style set

| Name | What |
|---|---|
| `Quag.Text` / `.Muted` / `.Faint` | Body text (editor's color), secondary, disabled |
| `Quag.Text.Title` | ◆ brand title, Quag blue, bold 13 |
| `Quag.Text.Heading` | Section label, Body blue, bold |
| `Quag.Text.Tag` | Gill purple, 9pt |
| `Quag.Text.Ok/Warn/Err/Info` | Status text (use `MakeStatus`, which adds the icon) |
| `Quag.Button.Primary/Secondary/Danger/Ghost` | `FButtonStyle`s |
| `Quag.TableRow` | `FTableRowStyle`: Quag-tinted selection, outline on hover |
| `Quag.Card` / `.Focused` | Editor panel color, 6px radius, 1px outline (Quag outline when focused) |
| `Quag.Tag`, `Quag.Row.Selected`, `Quag.Row.Hover`, `Quag.AccentBar` | Brushes |
| `Quag.Window` | Quag `BG`, only for standalone windows / runtime UI |
| `SQuagMascot` | Animated mascot widget (private only) |

## Data parts

| Helper | Level | What |
|---|---|---|
| `MakeSeverityBadge(EQuagSeverity)` | atom | `CRIT` `HIGH` `MED` `LOW` `INFO`, fixed width so a column lines up |
| `MakeDeltaBadge(Change, bHigherIsBetter)` | atom | `▲ +12.0%` green when the change is good, red when bad |
| `MakeUsageBar(Fraction)` | atom | thin bar: Quag, Warn from `QuagConsts::UsageWarn`, Err at `UsageErr` |
| `MakeStat(Value, Label)` | molecule | KPI card; `–` while the value is empty |
| `MakeMeter(Label, Fraction, Value)` | molecule | label · bar · value, with `!` / `✗` past the lines |
| `MakePropertyList(Rows)` | molecule | muted keys, editor-colored values |
| `MakeWeightsEditor(Names, Weights, OnChanged)` | molecule | raw weights (spin boxes), % of total, odds `1 in N` |
| `MakeErrorState(What, Why, OnRetry)` | element | what failed, why, Retry |
| `MakeLoading(What)` | element | the editor's throbber + what is loading |

`QuagConsts` holds the shared numbers (usage lines, stale time, nudge steps, weight step, id length).

```cpp
// An asset audit row: severity, asset, budget meter
SNew(SHorizontalBox)
+ SHorizontalBox::Slot().AutoWidth()[ FQuagStyle::MakeSeverityBadge(EQuagSeverity::High) ]
+ SHorizontalBox::Slot().FillWidth(1.f).Padding(8.f, 0.f)[ SNew(STextBlock).Text(AssetName) ]
+ SHorizontalBox::Slot().FillWidth(1.f)
[
	FQuagStyle::MakeMeter(LOCTEXT("Tex", "Texture memory"),
		TAttribute<float>::CreateSP(this, &SAuditRow::TextureFraction), TextureText)
]
```

**Curves, timelines, node graphs:** use the engine's curve editor, Sequencer and graph editor (DESIGN.md §9),
with `QuagColors` for the curve / pin / wire colors you are allowed to set.

Not compiled in-engine yet (no engine on the machine where it was written); the colors and `QuagConsts` are
checked against the other stacks by `quag-theme/tests/system.rs`. If something fails to compile on your UE version, it's most likely
one of the `FStyleColors` names or a brush constructor overload.
