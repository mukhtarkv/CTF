import { cn } from "@/utils/classNameMerge";
import { forwardRef, type InputHTMLAttributes } from "react";

export interface InputProps
  extends Omit<InputHTMLAttributes<HTMLInputElement>, "disabled" | "size"> {
  variant?: "primary" | "default" | "minimal" | "secondary" | "danger";
  size?: "sm" | "md" | "lg" | "xl";
  isDisabled?: boolean;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      variant = "default",
      size = "md",
      isDisabled = false,
      ...props
    },
    ref,
  ) => {
    return (
      <input
        className={cn(
          "-outline-offset-1 font-medium bg-base flex flex-row justify-center items-center transition-colors",
          {
            "text-sm px-[8px] py-[4px] rounded-[4px]": size === "sm",
            "text-[14px] px-[16px] py-[8px] rounded-[6px]": size === "md",
            "text-base px-[20px] py-[10px] rounded-[6px]": size === "lg",
            "text-lg px-[24px] py-[12px] rounded-[8px]": size === "xl",
            "outline outline-gray-600/40 bg-transparent text-text hover:bg-gray-600/10 focus:bg-gray-600/10 focus:outline-gray-600/60":
              variant === "default",
            "outline outline-gray-600/40 bg-gray-50 text-text hover:bg-gray-100 focus:bg-gray-100 focus:outline-gray-600/60":
              variant === "secondary",
            "outline outline-rose-500/40 bg-transparent text-text hover:bg-rose-500/10 focus:bg-rose-500/10 focus:outline-rose-500/60":
              variant === "danger",
            "outline-none ring-2 ring-sky-400/40 bg-transparent text-text hover:bg-sky-400/10 focus:bg-sky-400/10 focus:ring-sky-400/60":
              variant === "primary",
            "outline-none bg-transparent border-b border-gray-600/40 rounded-none hover:border-gray-600/60 focus:border-gray-600/80":
              variant === "minimal",
            "opacity-70 cursor-not-allowed": isDisabled,
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

Input.displayName = "Input";
export default Input;
