// Quag house style for Unreal Editor plugins (Slate), organised as the atomic system (STRUCTURE.md):
//   ATOMS      MakeButton, MakeTag, MakeStatus, MakeHeading, MakeSeverityBadge, MakeDeltaBadge, MakeUsageBar,
//              Quag.Text.* / Quag.Button.* / Quag.Card
//   MOLECULES  MakeButtonGroup, MakeTitle, MakeCard, MakeStat, MakeMeter, MakePropertyList, MakeWeightsEditor
//   ELEMENTS   MakeEmptyState, MakeErrorState, MakeLoading (shared); your plugin's own elements live in Private/Widgets/Elements/
//   UI         your tab spawner, which only places elements
// Rules: /home/primitation/APP-UI/DESIGN.md §9. In short:
//   * Backgrounds, fonts and inputs stay the EDITOR's (FAppStyle / FStyleColors), so the plugin feels native.
//   * Quag adds the accent (primary button, focus, selection in my own lists), the brand title,
//     text roles, status colors, tags, and the mascot in PRIVATE plugins only.

#pragma once

#include "CoreMinimal.h"
#include "Styling/SlateStyle.h"
#include "Styling/SlateColor.h"
#include "Widgets/SWidget.h"
#include "Input/Reply.h"

class SButton;
struct FSlateBrush;

/** Raw palette (sRGB hex -> linear, which is what Slate wants). Prefer the style names below. */
namespace QuagColors
{
	inline FLinearColor Hex(uint8 R, uint8 G, uint8 B, uint8 A = 255) { return FLinearColor(FColor(R, G, B, A)); }

	// Water (only for standalone windows / runtime UI; editor panels use FStyleColors)
	inline FLinearColor Bg()       { return Hex(0x0E, 0x16, 0x20); }
	inline FLinearColor Surface()  { return Hex(0x15, 0x22, 0x30); }
	inline FLinearColor Raised()   { return Hex(0x1C, 0x2D, 0x3F); }
	inline FLinearColor Border()   { return Hex(0x2A, 0x3F, 0x55); }
	// Ink
	inline FLinearColor Text()     { return Hex(0xE4, 0xF3, 0xFB); }
	inline FLinearColor Muted()    { return Hex(0x93, 0xAD, 0xC2); }
	inline FLinearColor Faint()    { return Hex(0x5F, 0x77, 0x8D); }
	// Quagsire
	inline FLinearColor Quag()     { return Hex(0x84, 0xD6, 0xF7); }
	inline FLinearColor QuagHi()   { return Hex(0xAD, 0xE7, 0xFF); }
	inline FLinearColor QuagSoft() { return Hex(0x84, 0xD6, 0xF7, 0x2E); } // ~18% tint for selected rows
	inline FLinearColor Body()     { return Hex(0x63, 0x9C, 0xBD); }
	inline FLinearColor Gill()     { return Hex(0xA8, 0x97, 0xD6); }
	inline FLinearColor GillSoft() { return Hex(0xA8, 0x97, 0xD6, 0x26); }
	// Status
	inline FLinearColor Ok()       { return Hex(0x7D, 0xD3, 0xA8); }
	inline FLinearColor Warn()     { return Hex(0xF2, 0xC4, 0x6D); }
	inline FLinearColor Err()      { return Hex(0xF2, 0x7D, 0x7F); }
}

/** Shared numbers (same values as the Rust `consts` and Solid `constants.ts`; tests/system.rs checks them). */
namespace QuagConsts
{
	/** Usage of a limit / budget: yellow from UsageWarn, red at UsageErr. */
	constexpr float UsageWarn = 0.8f;
	constexpr float UsageErr = 1.0f;
	/** Live data older than this shows "stale" in yellow. */
	constexpr int32 StaleAfterSecs = 30;
	/** Editing by keys: arrows move by Nudge of the value's range, Shift + arrows by NudgeFine. */
	constexpr float Nudge = 0.05f;
	constexpr float NudgeFine = 0.01f;
	/** Weights editor: one spin step of a raw weight. */
	constexpr float WeightStep = 1.0f;
	/** Ids / hashes are shown this many characters long. */
	constexpr int32 ShortIdLen = 12;
}

enum class EQuagButton : uint8 { Primary, Secondary, Danger, Ghost };

/** Findings (asset audits, lint, validation): worst first. */
enum class EQuagSeverity : uint8 { Critical, High, Medium, Low, Info };

/** One button inside a MakeButtonGroup (molecule). */
struct FQuagButtonSpec
{
	FText Label;
	EQuagButton Kind = EQuagButton::Secondary;
	FSimpleDelegate OnClicked;
	FText ToolTip;
	TAttribute<bool> IsEnabled = true;
};
enum class EQuagStatus : uint8 { Ok, Warn, Err, Info };

/**
 * Style set "QuagStyle". Names you can use with FQuagStyle::Get():
 *   Text:    Quag.Text  Quag.Text.Muted  Quag.Text.Faint  Quag.Text.Title  Quag.Text.Heading
 *            Quag.Text.Tag  Quag.Text.Ok  Quag.Text.Warn  Quag.Text.Err  Quag.Text.Info
 *   Buttons: Quag.Button.Primary  Quag.Button.Secondary  Quag.Button.Danger  Quag.Button.Ghost
 *   Rows:    Quag.TableRow (FTableRowStyle for SListView / STreeView rows)
 *   Brushes: Quag.Card  Quag.Card.Focused  Quag.Row.Selected  Quag.Row.Hover  Quag.Tag  Quag.AccentBar
 *            Quag.Window (standalone windows only)  Quag.Mascot.NN (private only)
 */
class QUAGSTYLE_API FQuagStyle
{
public:
	static void Initialize();
	static void Shutdown();

	/** Lazily initializes, so it is safe to call from any plugin that depends on QuagStyle. */
	static const ISlateStyle& Get();
	static FName GetStyleSetName();

	/** True if Resources/Mascot exists. Public plugins ship without that folder, so this is false. */
	static bool HasMascot();
	static int32 MascotFrameCount();
	static const FSlateBrush* MascotFrame(int32 Index);
	static float MascotFrameSeconds(int32 Index);

	// ── Widget helpers (use these instead of hand-styling) ──

	/** "◆ Title" in Quag blue, bold. One per tab/window, top-left. */
	static TSharedRef<SWidget> MakeTitle(const FText& Title, const FText& Subtitle = FText::GetEmpty());

	/** Section label (Body blue, bold). */
	static TSharedRef<SWidget> MakeHeading(const FText& Text);

	/** Button. Max ONE Primary per panel/dialog. */
	static TSharedRef<SButton> MakeButton(const FText& Label, EQuagButton Kind, FSimpleDelegate OnClicked,
	                                       const FText& ToolTip = FText::GetEmpty());

	/**
	 * MOLECULE: an array of buttons in a row, optionally in a box.
	 * Order rule (Unreal/desktop): put the Primary LAST (rightmost). A Danger button gets extra space before it.
	 * The ELEMENT that uses it decides which buttons exist and what they do.
	 */
	static TSharedRef<SWidget> MakeButtonGroup(const TArray<FQuagButtonSpec>& Buttons, bool bAlignRight = true, bool bBoxed = false);

	/** "✓ Saved" style status line. Always icon + color. */
	static TSharedRef<SWidget> MakeStatus(EQuagStatus Kind, const FText& Message);

	/** Small purple pill, for tags / badges. */
	static TSharedRef<SWidget> MakeTag(const FText& Text);

	/** Card container: editor panel color, rounded, 1px outline (Quag outline when bFocused). */
	static TSharedRef<SWidget> MakeCard(TSharedRef<SWidget> Content, bool bFocused = false);

	/** Empty state: mascot (if private + available), message, optional hint. */
	static TSharedRef<SWidget> MakeEmptyState(const FText& Message, const FText& Hint = FText::GetEmpty());

	// ── Data parts (same rules as DESIGN.md §2 "Severity", "Numbers", "Game design data") ──

	/** ATOM: fixed-width severity label: CRIT / HIGH (Err), MED (Warn), LOW (Body), INFO (Faint). Word + color, never color alone. */
	static TSharedRef<SWidget> MakeSeverityBadge(EQuagSeverity Severity);

	/** ATOM: "▲ +12.0%" / "▼ -3.5%" against a previous value. Green when the change is good, red when bad:
	 *  pass bHigherIsBetter = false for costs, load times, memory. |Change| < 0.05% shows nothing. */
	static TSharedRef<SWidget> MakeDeltaBadge(float Change, bool bHigherIsBetter);

	/** ATOM: thin bar of a used fraction: Quag, Warn from QuagConsts::UsageWarn, Err at UsageErr. Live via the attribute. */
	static TSharedRef<SWidget> MakeUsageBar(TAttribute<float> Fraction);

	/** MOLECULE: big number + label (a dashboard KPI). Shows "–" while Value is empty (data not there yet). */
	static TSharedRef<SWidget> MakeStat(const FText& Value, const FText& Label);

	/** MOLECULE: label · usage bar · value ("Texture memory  ━━━━━━  1.8 / 2 GiB"), with an icon past the warn line. */
	static TSharedRef<SWidget> MakeMeter(const FText& Label, TAttribute<float> Fraction, const FText& Value);

	/** MOLECULE: key / value rows (muted keys, editor-colored values). */
	static TSharedRef<SWidget> MakePropertyList(const TArray<TPair<FText, FText>>& Rows);

	/**
	 * MOLECULE: weighted choices (loot, spawns). Raw weights are edited (spin boxes); each row also shows its
	 * share of the total (always sums to 100 %) and its odds "1 in N". OnWeightChanged gets (row, new weight).
	 * The owner keeps the weights and rebuilds or invalidates when they change.
	 */
	static TSharedRef<SWidget> MakeWeightsEditor(const TArray<FText>& Names, TAttribute<TArray<float>> Weights,
	                                              TFunction<void(int32, float)> OnWeightChanged);

	/** ELEMENT: what went wrong, why (muted), and a Retry button. Never a popup. */
	static TSharedRef<SWidget> MakeErrorState(const FText& What, const FText& Why, FSimpleDelegate OnRetry = FSimpleDelegate());

	/** ELEMENT: loading: the editor's own throbber + what is loading. */
	static TSharedRef<SWidget> MakeLoading(const FText& What);

private:
	static TSharedRef<class FSlateStyleSet> Create();
	static TSharedPtr<class FSlateStyleSet> StyleInstance;
	static int32 NumMascotFrames;
};
