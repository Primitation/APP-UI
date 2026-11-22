#include "SQuagMascot.h"
#include "QuagStyle.h"

#include "Widgets/Images/SImage.h"
#include "Widgets/Layout/SBox.h"

void SQuagMascot::Construct(const FArguments& InArgs)
{
	if (!FQuagStyle::HasMascot())
	{
		ChildSlot[ SNullWidget::NullWidget ];
		return;
	}

	const float Width = InArgs._Size;
	const float Height = Width * (382.f / 400.f);

	ChildSlot
	[
		SNew(SBox)
		.WidthOverride(Width)
		.HeightOverride(Height)
		[
			SAssignNew(Image, SImage)
			.Image(FQuagStyle::MascotFrame(0))
		]
	];

	if (InArgs._Animate && FQuagStyle::MascotFrameCount() > 1)
	{
		// Tick at ~30 Hz; frames advance by their own GIF delays.
		RegisterActiveTimer(1.f / 30.f, FWidgetActiveTimerDelegate::CreateSP(this, &SQuagMascot::Advance));
	}
}

EActiveTimerReturnType SQuagMascot::Advance(double /*InCurrentTime*/, float InDeltaTime)
{
	const int32 Count = FQuagStyle::MascotFrameCount();
	if (Count <= 1 || !Image.IsValid())
	{
		return EActiveTimerReturnType::Stop;
	}

	Elapsed += InDeltaTime;
	const int32 Before = Frame;
	while (Elapsed >= FQuagStyle::MascotFrameSeconds(Frame))
	{
		Elapsed -= FQuagStyle::MascotFrameSeconds(Frame);
		Frame = (Frame + 1) % Count;
	}
	if (Frame != Before)
	{
		Image->SetImage(FQuagStyle::MascotFrame(Frame));
	}
	return EActiveTimerReturnType::Continue;
}
