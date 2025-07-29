import { cn } from "@/utils/classNameMerge";
import { forwardRef, type ButtonHTMLAttributes } from "react";

export interface ButtonProps
  extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "disabled"> {
  variant?: "primary" | "default" | "minimal" | "secondary" | "danger";
  size?: "sm" | "md" | "lg" | "xl";
  isDisabled?: boolean;
  isIcon?: boolean;
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      variant = "default",
      size = "md",
      isDisabled = false,
      isIcon = false,
      ...props
    },
    ref,
  ) => {
    return (
      <button
        className={cn(
          "-outline-offset-1 font-medium bg-base flex flex-row justify-center items-center",
          {
            "text-sm px-[8px] py-[4px] rounded-[4px] gap-1": size === "sm",
            "text-[14px] px-[16px] py-[8px] rounded-[6px] gap-1": size === "md",
            "text-base px-[20px] py-[10px] rounded-[6px] gap-2": size === "lg",
            "text-lg px-[24px] py-[12px] rounded-[8px] gap-2": size === "xl",

            "outline outline-gray-600/40 bg-transparent text-text hover:bg-gray-600/20 active:bg-gray-600/30":
              variant === "default",
            "outline-none bg-gray-600 text-white hover:bg-gray-700 active:bg-gray-800":
              variant === "secondary",
            "outline-none bg-rose-500 text-white hover:bg-rose-600 active:bg-rose-700":
              variant === "danger",
            "outline-none bg-sky-600 text-white hover:bg-sky-700 active:bg-sky-800":
              variant === "primary",
            "outline-none bg-transparent hover:bg-white/20 active:bg-white/30":
              variant === "minimal",

            "opacity-70 cursor-not-allowed": isDisabled,

            "text-sm px-[6px] py-[4px] rounded-[4px] gap-1":
              size === "sm" && isIcon,
            "text-[14px] px-[8px] py-[8px] rounded-[6px] gap-1":
              size === "md" && isIcon,
            "text-base px-[10px] py-[10px] rounded-[6px] gap-2":
              size === "lg" && isIcon,
            "text-lg px-[12px] py-[12px] rounded-[8px] gap-2":
              size === "xl" && isIcon,
          },
          className,
        )}
        ref={ref}
        disabled={isDisabled}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

export default Button;
