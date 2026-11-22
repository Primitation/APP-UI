#include "QuagStyle.h"
#include "SQuagMascot.h"

#include "Brushes/SlateColorBrush.h"
#include "Brushes/SlateImageBrush.h"
#include "Brushes/SlateNoResource.h"
#include "Brushes/SlateRoundedBoxBrush.h"
#include "HAL/FileManager.h"
#include "Interfaces/IPluginManager.h"
#include "Misc/FileHelper.h"
#include "Misc/Paths.h"
#include "Styling/AppStyle.h"
#include "Styling/CoreStyle.h"
#include "Styling/SlateStyleRegistry.h"
#include "Styling/SlateTypes.h"
#include "Styling/StyleColors.h"
#include "Widgets/Images/SImage.h"
#include "Widgets/Images/SThrobber.h"
#include "Widgets/Input/SNumericEntryBox.h"
#include "Widgets/Notifications/SProgressBar.h"
#include "Widgets/Input/SButton.h"
#include "Widgets/Layout/SBorder.h"
#include "Widgets/Layout/SBox.h"
#include "Widgets/SBoxPanel.h"
#include "Widgets/SNullWidget.h"
#include "Widgets/Text/STextBlock.h"

TSharedPtr<FSlateStyleSet> FQuagStyle::StyleInstance = nullptr;
int32 FQuagStyle::NumMascotFrames = 0;

static TArray<float> GMascotDelays;

FName FQuagStyle::GetStyleSetName()
{
	static const FName Name(TEXT("QuagStyle"));
	return Name;
}

void FQuagStyle::Initialize()
{
	if (!StyleInstance.IsValid())
	{
		StyleInstance = Create();
		FSlateStyleRegistry::RegisterSlateStyle(*StyleInstance);
	}
}

void FQuagStyle::Shutdown()
{
	if (StyleInstance.IsValid())
	{
		FSlateStyleRegistry::UnRegisterSlateStyle(*StyleInstance);
		StyleInstance.Reset();
	}
}

const ISlateStyle& FQuagStyle::Get()
{
	Initialize();
	return *StyleInstance;
}

// ───────────────────────────── Style set ─────────────────────────────

TSharedRef<FSlateStyleSet> FQuagStyle::Create()
{
	using namespace QuagColors;
	TSharedRef<FSlateStyleSet> Style = MakeShared<FSlateStyleSet>(GetStyleSetName());

	FString Resources;
	if (TSharedPtr<IPlugin> Plugin = IPluginManager::Get().FindPlugin(TEXT("QuagStyle")))
	{
		Resources = Plugin->GetBaseDir() / TEXT("Resources");
	}
	Style->SetContentRoot(Resources);

	// ── Text: start from the editor's NormalText so size/font match the editor ──
	const FTextBlockStyle Normal = FAppStyle::Get().GetWidgetStyle<FTextBlockStyle>("NormalText");
	const FSlateFontInfo Regular = FCoreStyle::GetDefaultFontStyle("Regular", 10);
	const FSlateFontInfo Bold    = FCoreStyle::GetDefaultFontStyle("Bold", 10);

	auto TextStyle = [&](const FSlateFontInfo& Font, const FSlateColor& Color)
	{
		return FTextBlockStyle(Normal).SetFont(Font).SetColorAndOpacity(Color);
	};
	Style->Set("Quag.Text",         TextStyle(Regular, FStyleColors::Foreground));   // editor's own text color
	Style->Set("Quag.Text.Muted",   TextStyle(Regular, FSlateColor(Muted())));
	Style->Set("Quag.Text.Faint",   TextStyle(Regular, FSlateColor(Faint())));
	Style->Set("Quag.Text.Title",   TextStyle(FCoreStyle::GetDefaultFontStyle("Bold", 13), FSlateColor(Quag())));
	Style->Set("Quag.Text.Heading", TextStyle(Bold, FSlateColor(Body())));
	Style->Set("Quag.Text.Tag",     TextStyle(FCoreStyle::GetDefaultFontStyle("Regular", 9), FSlateColor(Gill())));
	Style->Set("Quag.Text.Ok",      TextStyle(Regular, FSlateColor(Ok())));
	Style->Set("Quag.Text.Warn",    TextStyle(Regular, FSlateColor(Warn())));
	Style->Set("Quag.Text.Err",     TextStyle(Regular, FSlateColor(Err())));
	Style->Set("Quag.Text.Info",    TextStyle(Regular, FSlateColor(Body())));
	// Button labels take the button's foreground (set per state below)
	Style->Set("Quag.Text.Button",  TextStyle(Bold, FSlateColor::UseForeground()));

	// ── Buttons ──
	constexpr float R = 4.f; // same radius the editor uses
	const FMargin Pad(16.f, 5.f, 16.f, 5.f);
	const FMargin PadPressed(16.f, 6.f, 16.f, 4.f);

	Style->Set("Quag.Button.Primary", FButtonStyle()
		.SetNormal  (FSlateRoundedBoxBrush(Quag(),   R))
		.SetHovered (FSlateRoundedBoxBrush(QuagHi(), R))
		.SetPressed (FSlateRoundedBoxBrush(Body(),   R))
		.SetDisabled(FSlateRoundedBoxBrush(FStyleColors::Dropdown, R))
		.SetNormalForeground(FSlateColor(Bg()))
		.SetHoveredForeground(FSlateColor(Bg()))
		.SetPressedForeground(FSlateColor(Bg()))
		.SetDisabledForeground(FStyleColors::Foreground)
		.SetNormalPadding(Pad).SetPressedPadding(PadPressed));

	const FButtonStyle Secondary = FButtonStyle()
		.SetNormal  (FSlateRoundedBoxBrush(FStyleColors::Dropdown, R, FStyleColors::Input, 1.f))
		.SetHovered (FSlateRoundedBoxBrush(FStyleColors::Hover,    R, FStyleColors::Input, 1.f))
		.SetPressed (FSlateRoundedBoxBrush(FStyleColors::Header,   R, FStyleColors::Input, 1.f))
		.SetDisabled(FSlateRoundedBoxBrush(FStyleColors::Dropdown, R))
		.SetNormalForeground(FStyleColors::ForegroundHover)
		.SetHoveredForeground(FStyleColors::ForegroundHover)
		.SetPressedForeground(FStyleColors::ForegroundHover)
		.SetDisabledForeground(FSlateColor(Faint()))
		.SetNormalPadding(Pad).SetPressedPadding(PadPressed);
	Style->Set("Quag.Button.Secondary", Secondary);

	Style->Set("Quag.Button.Danger", FButtonStyle(Secondary)
		.SetNormalForeground(FSlateColor(Err()))
		.SetHoveredForeground(FSlateColor(Err()))
		.SetPressedForeground(FSlateColor(Err())));

	Style->Set("Quag.Button.Ghost", FButtonStyle()
		.SetNormal  (FSlateNoResource())
		.SetHovered (FSlateRoundedBoxBrush(FStyleColors::Hover, R))
		.SetPressed (FSlateRoundedBoxBrush(FStyleColors::Header, R))
		.SetDisabled(FSlateNoResource())
		.SetNormalForeground(FSlateColor(Muted()))
		.SetHoveredForeground(FStyleColors::ForegroundHover)
		.SetPressedForeground(FStyleColors::ForegroundHover)
		.SetDisabledForeground(FSlateColor(Faint()))
		.SetNormalPadding(Pad).SetPressedPadding(PadPressed));

	// ── List / tree rows: use with STableRow<...>(OwnerTable).Style(&FQuagStyle::Get().GetWidgetStyle<FTableRowStyle>("Quag.TableRow")) ──
	const FTableRowStyle BaseRow = FAppStyle::Get().GetWidgetStyle<FTableRowStyle>("TableView.Row");
	Style->Set("Quag.TableRow", FTableRowStyle(BaseRow)
		.SetActiveBrush(FSlateRoundedBoxBrush(FSlateColor(QuagSoft()), 4.f))
		.SetActiveHoveredBrush(FSlateRoundedBoxBrush(FSlateColor(QuagSoft()), 4.f, FSlateColor(Quag()), 1.f))
		.SetInactiveBrush(FSlateRoundedBoxBrush(FStyleColors::SelectInactive, 4.f))
		.SetInactiveHoveredBrush(FSlateRoundedBoxBrush(FStyleColors::SelectInactive, 4.f))
		.SetSelectedTextColor(FSlateColor(QuagHi())));

	// ── Brushes ──
	Style->Set("Quag.Card",         new FSlateRoundedBoxBrush(FStyleColors::Panel, 6.f, FStyleColors::Input, 1.f));
	Style->Set("Quag.Card.Focused", new FSlateRoundedBoxBrush(FStyleColors::Panel, 6.f, FSlateColor(Quag()), 1.f));
	Style->Set("Quag.Row.Selected", new FSlateRoundedBoxBrush(FSlateColor(QuagSoft()), 4.f));
	Style->Set("Quag.Row.Hover",    new FSlateRoundedBoxBrush(FStyleColors::Hover, 4.f));
	Style->Set("Quag.Tag",          new FSlateRoundedBoxBrush(FSlateColor(GillSoft()), 8.f));
	Style->Set("Quag.AccentBar",    new FSlateColorBrush(Quag()));   // 3px bar left of a selected row
	Style->Set("Quag.Window",       new FSlateColorBrush(Bg()));     // standalone windows / runtime only

	// ── Mascot (only if Resources/Mascot was shipped = private plugin) ──
	NumMascotFrames = 0;
	GMascotDelays.Reset();
	const FString MascotDir = Resources / TEXT("Mascot");
	while (FPaths::FileExists(MascotDir / FString::Printf(TEXT("quag_%02d.png"), NumMascotFrames)))
	{
		Style->Set(*FString::Printf(TEXT("Quag.Mascot.%02d"), NumMascotFrames),
			new FSlateImageBrush(Style->RootToContentDir(FString::Printf(TEXT("Mascot/quag_%02d"), NumMascotFrames), TEXT(".png")),
				FVector2f(400.f, 382.f)));
		++NumMascotFrames;
	}
	TArray<FString> Lines;
	if (NumMascotFrames > 0 && FFileHelper::LoadFileToStringArray(Lines, *(MascotDir / TEXT("delays.txt"))))
	{
		for (const FString& L : Lines)
		{
			if (!L.IsEmpty()) { GMascotDelays.Add(FCString::Atof(*L) / 1000.f); }
		}
	}

	return Style;
}

bool FQuagStyle::HasMascot()                { Initialize(); return NumMascotFrames > 0; }
int32 FQuagStyle::MascotFrameCount()        { Initialize(); return NumMascotFrames; }

const FSlateBrush* FQuagStyle::MascotFrame(int32 Index)
{
	if (!HasMascot()) { return nullptr; }
	return Get().GetBrush(*FString::Printf(TEXT("Quag.Mascot.%02d"), Index % NumMascotFrames));
}

float FQuagStyle::MascotFrameSeconds(int32 Index)
{
	return GMascotDelays.IsValidIndex(Index) ? FMath::Max(GMascotDelays[Index], 0.02f) : 0.1f;
}

// ───────────────────────────── Widget helpers ─────────────────────────────

TSharedRef<SWidget> FQuagStyle::MakeTitle(const FText& Title, const FText& Subtitle)
{
	return SNew(SHorizontalBox)
		+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center)
		[
			SNew(STextBlock)
			.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Title"))
			.Text(FText::Format(INVTEXT("◆ {0}"), Title))
		]
		+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center).Padding(10.f, 0.f, 0.f, 0.f)
		[
			SNew(STextBlock)
			.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Muted"))
			.Text(Subtitle)
			.Visibility(Subtitle.IsEmpty() ? EVisibility::Collapsed : EVisibility::Visible)
		];
}

TSharedRef<SWidget> FQuagStyle::MakeHeading(const FText& Text)
{
	return SNew(STextBlock)
		.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Heading"))
		.Text(Text);
}

TSharedRef<SButton> FQuagStyle::MakeButton(const FText& Label, EQuagButton Kind, FSimpleDelegate OnClicked, const FText& ToolTip)
{
	const TCHAR* StyleName = TEXT("Quag.Button.Secondary");
	switch (Kind)
	{
		case EQuagButton::Primary:   StyleName = TEXT("Quag.Button.Primary"); break;
		case EQuagButton::Danger:    StyleName = TEXT("Quag.Button.Danger");  break;
		case EQuagButton::Ghost:     StyleName = TEXT("Quag.Button.Ghost");   break;
		case EQuagButton::Secondary: break;
	}
	return SNew(SButton)
		.ButtonStyle(&Get().GetWidgetStyle<FButtonStyle>(StyleName))
		.ToolTipText(ToolTip)
		.OnClicked_Lambda([OnClicked]() { OnClicked.ExecuteIfBound(); return FReply::Handled(); })
		[
			SNew(STextBlock)
			.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Button"))
			.ColorAndOpacity(FSlateColor::UseForeground())
			.Text(Label)
		];
}

TSharedRef<SWidget> FQuagStyle::MakeButtonGroup(const TArray<FQuagButtonSpec>& Buttons, bool bAlignRight, bool bBoxed)
{
	TSharedRef<SHorizontalBox> Row = SNew(SHorizontalBox);
	for (int32 i = 0; i < Buttons.Num(); ++i)
	{
		const FQuagButtonSpec& B = Buttons[i];
		// 8px between buttons; 16px before a destructive one so it isn't hit by accident
		const float Left = (i == 0) ? 0.f : (B.Kind == EQuagButton::Danger ? 16.f : 8.f);
		TSharedRef<SButton> Button = MakeButton(B.Label, B.Kind, B.OnClicked, B.ToolTip);
		Button->SetEnabled(B.IsEnabled);
		Row->AddSlot().AutoWidth().Padding(Left, 0.f, 0.f, 0.f)[ Button ];
	}

	TSharedRef<SWidget> Aligned = SNew(SBox).HAlign(bAlignRight ? HAlign_Right : HAlign_Left)[ Row ];
	if (!bBoxed)
	{
		return Aligned;
	}
	return SNew(SBorder)
		.BorderImage(Get().GetBrush("Quag.Card"))
		.Padding(12.f)
		[
			Aligned
		];
}

TSharedRef<SWidget> FQuagStyle::MakeStatus(EQuagStatus Kind, const FText& Message)
{
	const TCHAR* StyleName = TEXT("Quag.Text.Info");
	const TCHAR* Icon = TEXT("i");
	switch (Kind)
	{
		case EQuagStatus::Ok:   StyleName = TEXT("Quag.Text.Ok");   Icon = TEXT("✓"); break;
		case EQuagStatus::Warn: StyleName = TEXT("Quag.Text.Warn"); Icon = TEXT("!");      break;
		case EQuagStatus::Err:  StyleName = TEXT("Quag.Text.Err");  Icon = TEXT("✗"); break;
		case EQuagStatus::Info: break;
	}
	return SNew(STextBlock)
		.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>(StyleName))
		.Text(FText::Format(INVTEXT("{0} {1}"), FText::FromString(Icon), Message));
}

TSharedRef<SWidget> FQuagStyle::MakeTag(const FText& Text)
{
	return SNew(SBorder)
		.BorderImage(Get().GetBrush("Quag.Tag"))
		.Padding(FMargin(8.f, 1.f))
		.VAlign(VAlign_Center)
		[
			SNew(STextBlock)
			.TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Tag"))
			.Text(Text)
		];
}

TSharedRef<SWidget> FQuagStyle::MakeCard(TSharedRef<SWidget> Content, bool bFocused)
{
	return SNew(SBorder)
		.BorderImage(Get().GetBrush(bFocused ? "Quag.Card.Focused" : "Quag.Card"))
		.Padding(12.f)
		[
			Content
		];
}

TSharedRef<SWidget> FQuagStyle::MakeEmptyState(const FText& Message, const FText& Hint)
{
	TSharedRef<SVerticalBox> Box = SNew(SVerticalBox);
	if (HasMascot())
	{
		Box->AddSlot().AutoHeight().HAlign(HAlign_Center).Padding(0.f, 0.f, 0.f, 12.f)
		[
			SNew(SQuagMascot).Size(160.f)
		];
	}
	Box->AddSlot().AutoHeight().HAlign(HAlign_Center)
	[
		SNew(STextBlock).TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text")).Text(Message)
	];
	if (!Hint.IsEmpty())
	{
		Box->AddSlot().AutoHeight().HAlign(HAlign_Center).Padding(0.f, 4.f, 0.f, 0.f)
		[
			SNew(STextBlock).TextStyle(&Get().GetWidgetStyle<FTextBlockStyle>("Quag.Text.Muted")).Text(Hint)
		];
	}
	return SNew(SBox).HAlign(HAlign_Center).VAlign(VAlign_Center).Padding(24.f)[ Box ];
}

// ───────────────────────────── Data parts ─────────────────────────────

namespace
{
	const FTextBlockStyle& TextStyleOf(const TCHAR* Name) { return FQuagStyle::Get().GetWidgetStyle<FTextBlockStyle>(Name); }

	/** Color of a used fraction (QuagConsts thresholds). */
	FLinearColor UsageColor(float Fraction)
	{
		if (Fraction >= QuagConsts::UsageErr)  { return QuagColors::Err(); }
		if (Fraction >= QuagConsts::UsageWarn) { return QuagColors::Warn(); }
		return QuagColors::Quag();
	}

	/** "2.4" / "12" / "1 250": short numbers for odds and shares. */
	FText ShortNumber(float V)
	{
		FNumberFormattingOptions O;
		O.MaximumFractionalDigits = V < 10.f ? 2 : (V < 100.f ? 1 : 0);
		return FText::AsNumber(V, &O);
	}
}

TSharedRef<SWidget> FQuagStyle::MakeSeverityBadge(EQuagSeverity Severity)
{
	const TCHAR* Word = TEXT("INFO");
	const TCHAR* StyleName = TEXT("Quag.Text.Faint");
	switch (Severity)
	{
		case EQuagSeverity::Critical: Word = TEXT("CRIT"); StyleName = TEXT("Quag.Text.Err");  break;
		case EQuagSeverity::High:     Word = TEXT("HIGH"); StyleName = TEXT("Quag.Text.Err");  break;
		case EQuagSeverity::Medium:   Word = TEXT("MED");  StyleName = TEXT("Quag.Text.Warn"); break;
		case EQuagSeverity::Low:      Word = TEXT("LOW");  StyleName = TEXT("Quag.Text.Info"); break;
		case EQuagSeverity::Info:     break;
	}
	// fixed width so a column of badges lines up
	return SNew(SBox).WidthOverride(44.f)
	[
		SNew(STextBlock).TextStyle(&TextStyleOf(StyleName)).Font(FCoreStyle::GetDefaultFontStyle("Bold", 9)).Text(FText::FromString(Word))
	];
}

TSharedRef<SWidget> FQuagStyle::MakeDeltaBadge(float Change, bool bHigherIsBetter)
{
	if (FMath::Abs(Change) < 0.0005f)
	{
		return SNullWidget::NullWidget;
	}
	const bool bUp = Change > 0.f;
	const bool bGood = (bUp == bHigherIsBetter);
	FNumberFormattingOptions O;
	O.MinimumFractionalDigits = 1;
	O.MaximumFractionalDigits = 1;
	O.AlwaysSign = true;
	return SNew(STextBlock)
		.TextStyle(&TextStyleOf(bGood ? TEXT("Quag.Text.Ok") : TEXT("Quag.Text.Err")))
		.Text(FText::Format(INVTEXT("{0} {1}%"), FText::FromString(bUp ? TEXT("▲") : TEXT("▼")), FText::AsNumber(Change * 100.f, &O)));
}

TSharedRef<SWidget> FQuagStyle::MakeUsageBar(TAttribute<float> Fraction)
{
	return SNew(SBox).HeightOverride(4.f).VAlign(VAlign_Center)
	[
		SNew(SProgressBar)
		.Percent_Lambda([Fraction]() { return TOptional<float>(FMath::Clamp(Fraction.Get(), 0.f, 1.f)); })
		.FillColorAndOpacity_Lambda([Fraction]() { return FSlateColor(UsageColor(Fraction.Get())); })
	];
}

TSharedRef<SWidget> FQuagStyle::MakeStat(const FText& Value, const FText& Label)
{
	return MakeCard(
		SNew(SVerticalBox)
		+ SVerticalBox::Slot().AutoHeight()
		[
			SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text"))).Font(FCoreStyle::GetDefaultFontStyle("Bold", 16))
			.Text(Value.IsEmpty() ? INVTEXT("–") : Value)
		]
		+ SVerticalBox::Slot().AutoHeight().Padding(0.f, 2.f, 0.f, 0.f)
		[
			SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted"))).Text(Label)
		]);
}

TSharedRef<SWidget> FQuagStyle::MakeMeter(const FText& Label, TAttribute<float> Fraction, const FText& Value)
{
	return SNew(SHorizontalBox)
		+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center).Padding(0.f, 0.f, 12.f, 0.f)
		[
			SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted"))).Text(Label)
		]
		+ SHorizontalBox::Slot().FillWidth(1.f).VAlign(VAlign_Center)
		[
			MakeUsageBar(Fraction)
		]
		+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center).Padding(12.f, 0.f, 0.f, 0.f)
		[
			// icon + color past the warn line, so it never relies on color alone
			SNew(STextBlock)
			.TextStyle(&TextStyleOf(TEXT("Quag.Text")))
			.ColorAndOpacity_Lambda([Fraction]() { return Fraction.Get() >= QuagConsts::UsageWarn ? FSlateColor(UsageColor(Fraction.Get())) : FSlateColor(FStyleColors::Foreground); })
			.Text_Lambda([Fraction, Value]()
			{
				const float F = Fraction.Get();
				return F >= QuagConsts::UsageErr ? FText::Format(INVTEXT("✗ {0}"), Value)
				     : F >= QuagConsts::UsageWarn ? FText::Format(INVTEXT("! {0}"), Value) : Value;
			})
		];
}

TSharedRef<SWidget> FQuagStyle::MakePropertyList(const TArray<TPair<FText, FText>>& Rows)
{
	TSharedRef<SVerticalBox> Box = SNew(SVerticalBox);
	for (const TPair<FText, FText>& Row : Rows)
	{
		Box->AddSlot().AutoHeight().Padding(0.f, 2.f)
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(0.4f)
			[
				SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted"))).Text(Row.Key)
			]
			+ SHorizontalBox::Slot().FillWidth(0.6f)
			[
				SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text"))).Text(Row.Value)
			]
		];
	}
	return Box;
}

TSharedRef<SWidget> FQuagStyle::MakeWeightsEditor(const TArray<FText>& Names, TAttribute<TArray<float>> Weights,
                                                  TFunction<void(int32, float)> OnWeightChanged)
{
	// share of row i in the total; 0 when everything is 0
	auto Share = [Weights](int32 i)
	{
		const TArray<float> W = Weights.Get();
		float Total = 0.f;
		for (float X : W) { Total += X; }
		return (Total > 0.f && W.IsValidIndex(i)) ? W[i] / Total : 0.f;
	};
	// the biggest share fills the bar; the others are relative to it
	auto TopShare = [Share, Weights]()
	{
		float Top = 0.f;
		for (int32 j = 0; j < Weights.Get().Num(); ++j) { Top = FMath::Max(Top, Share(j)); }
		return FMath::Max(Top, KINDA_SMALL_NUMBER);
	};
	TSharedRef<SVerticalBox> Box = SNew(SVerticalBox);
	for (int32 i = 0; i < Names.Num(); ++i)
	{
		Box->AddSlot().AutoHeight().Padding(0.f, 3.f)
		[
			SNew(SHorizontalBox)
			+ SHorizontalBox::Slot().FillWidth(0.3f).VAlign(VAlign_Center)
			[
				SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text"))).Text(Names[i])
			]
			+ SHorizontalBox::Slot().FillWidth(0.3f).VAlign(VAlign_Center).Padding(8.f, 0.f)
			[
				// a share is not a budget: always Quag, never yellow / red
				SNew(SBox).HeightOverride(4.f).VAlign(VAlign_Center)
				[
					SNew(SProgressBar)
					.Percent_Lambda([Share, TopShare, i]() { return TOptional<float>(Share(i) / TopShare()); })
					.FillColorAndOpacity(FSlateColor(QuagColors::Quag()))
				]
			]
			+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center)
			[
				SNew(SBox).WidthOverride(80.f)
				[
					SNew(SNumericEntryBox<float>)
					.MinValue(0.f)
					.Delta(QuagConsts::WeightStep)
					.AllowSpin(true)
					.Value_Lambda([Weights, i]() { const TArray<float> W = Weights.Get(); return W.IsValidIndex(i) ? TOptional<float>(W[i]) : TOptional<float>(); })
					.OnValueChanged_Lambda([OnWeightChanged, i](float V) { if (OnWeightChanged) { OnWeightChanged(i, FMath::Max(0.f, V)); } })
					.OnValueCommitted_Lambda([OnWeightChanged, i](float V, ETextCommit::Type) { if (OnWeightChanged) { OnWeightChanged(i, FMath::Max(0.f, V)); } })
				]
			]
			+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center).Padding(12.f, 0.f, 0.f, 0.f)
			[
				SNew(SBox).WidthOverride(56.f).HAlign(HAlign_Right)
				[
					SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text")))
					.Text_Lambda([Share, i]() { return FText::Format(INVTEXT("{0} %"), ShortNumber(Share(i) * 100.f)); })
				]
			]
			+ SHorizontalBox::Slot().AutoWidth().VAlign(VAlign_Center).Padding(12.f, 0.f, 0.f, 0.f)
			[
				SNew(SBox).WidthOverride(72.f).HAlign(HAlign_Right)
				[
					SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted")))
					.Text_Lambda([Share, i]() { const float S = Share(i); return S > 0.f ? FText::Format(INVTEXT("1 in {0}"), ShortNumber(1.f / S)) : INVTEXT("never"); })
				]
			]
		];
	}
	// the total line: 100 %, or a warning when nothing can be picked
	Box->AddSlot().AutoHeight().Padding(0.f, 8.f, 0.f, 0.f)
	[
		SNew(STextBlock)
		.TextStyle(&TextStyleOf(TEXT("Quag.Text.Faint")))
		.ColorAndOpacity_Lambda([Weights]() { float T = 0.f; for (float X : Weights.Get()) { T += X; } return FSlateColor(T > 0.f ? QuagColors::Faint() : QuagColors::Warn()); })
		.Text_Lambda([Weights]()
		{
			float T = 0.f;
			for (float X : Weights.Get()) { T += X; }
			return T > 0.f ? FText::Format(INVTEXT("{0} entries · weights {1} · 100 %"), Weights.Get().Num(), ShortNumber(T))
			               : INVTEXT("! nothing can be picked: every weight is 0");
		})
	];
	return Box;
}

TSharedRef<SWidget> FQuagStyle::MakeErrorState(const FText& What, const FText& Why, FSimpleDelegate OnRetry)
{
	TSharedRef<SVerticalBox> Box = SNew(SVerticalBox)
		+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Center)
		[
			MakeStatus(EQuagStatus::Err, What)
		]
		+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Center).Padding(0.f, 4.f, 0.f, 0.f)
		[
			SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted"))).Text(Why).AutoWrapText(true)
		];
	if (OnRetry.IsBound())
	{
		Box->AddSlot().AutoHeight().HAlign(HAlign_Center).Padding(0.f, 12.f, 0.f, 0.f)
		[
			MakeButton(INVTEXT("Retry"), EQuagButton::Secondary, OnRetry)
		];
	}
	return SNew(SBox).HAlign(HAlign_Center).VAlign(VAlign_Center).Padding(24.f)[ Box ];
}

TSharedRef<SWidget> FQuagStyle::MakeLoading(const FText& What)
{
	return SNew(SBox).HAlign(HAlign_Center).VAlign(VAlign_Center).Padding(24.f)
	[
		SNew(SVerticalBox)
		+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Center)
		[
			SNew(SThrobber)
		]
		+ SVerticalBox::Slot().AutoHeight().HAlign(HAlign_Center).Padding(0.f, 8.f, 0.f, 0.f)
		[
			SNew(STextBlock).TextStyle(&TextStyleOf(TEXT("Quag.Text.Muted"))).Text(What)
		]
	];
}
