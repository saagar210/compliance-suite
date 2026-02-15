import * as React from "react";

export type FormProps = React.FormHTMLAttributes<HTMLFormElement>;

const Form = React.forwardRef<HTMLFormElement, FormProps>(({ className = "", ...props }, ref) => {
  return <form ref={ref} className={`space-y-4 ${className}`} {...props} />;
});

Form.displayName = "Form";

export interface FormFieldProps {
  label?: string;
  error?: string;
  children: React.ReactNode;
  required?: boolean;
}

export function FormField({ label, error, children, required }: FormFieldProps) {
  return (
    <div className="space-y-2">
      {label && (
        <label className="block text-sm font-medium text-foreground">
          {label}
          {required && <span className="text-destructive ml-1">*</span>}
        </label>
      )}
      {children}
      {error && <p className="text-sm text-destructive">{error}</p>}
    </div>
  );
}

export default Form;
